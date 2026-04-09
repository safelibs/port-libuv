use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Deserialize)]
struct AbiBaseline {
    linux_x86_64: LinuxX8664Baseline,
}

#[derive(Debug, Deserialize)]
struct LinuxX8664Baseline {
    dynamic_exports: Vec<String>,
    soname: String,
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let repo_root = manifest_dir.parent().expect("repo root");
    let build_dir = repo_root.join("original/build-checker");
    let shared_link_txt = build_dir.join("CMakeFiles/uv.dir/link.txt");
    let static_link_txt = build_dir.join("CMakeFiles/uv_a.dir/link.txt");
    let abi_baseline = manifest_dir.join("tools/abi-baseline.json");

    for path in [
        repo_root.join("original/CMakeLists.txt"),
        repo_root.join("original/cmake"),
        repo_root.join("original/include"),
        repo_root.join("original/src"),
        build_dir.join("CMakeCache.txt"),
        shared_link_txt.clone(),
        static_link_txt.clone(),
        abi_baseline.clone(),
    ] {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    assert_supported_target();

    let baseline = load_abi_baseline(&abi_baseline);
    let shared_link_contents = fs::read_to_string(&shared_link_txt).expect("shared link.txt");
    let static_link_contents = fs::read_to_string(&static_link_txt).expect("static link.txt");

    let soname_flag = format!("-Wl,-soname,{}", baseline.linux_x86_64.soname);
    if !shared_link_contents.contains(&soname_flag) {
        panic!("baseline shared linker contract no longer carries {soname_flag}");
    }

    for lib in ["pthread", "dl", "rt"] {
        let link_flag = format!("-l{lib}");
        if !shared_link_contents.contains(&link_flag) {
            panic!("baseline shared linker contract no longer carries {link_flag}");
        }
        println!("cargo:rustc-link-lib={lib}");
    }

    let static_object_count = count_baseline_objects(&static_link_contents);
    if static_object_count == 0 {
        panic!("baseline static archive contract contains no object members");
    }
    println!("cargo:rustc-env=LIBUV_STATIC_BASELINE_OBJECT_COUNT={static_object_count}");

    build_upstream(&build_dir);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let version_script = out_dir.join("libuv.map");
    let rename_map = out_dir.join("libuv-internal-rename.map");
    let internal_archive = out_dir.join("libuv_internal.a");

    fs::write(
        &version_script,
        render_version_script(&baseline.linux_x86_64.dynamic_exports),
    )
    .expect("write version script");
    fs::write(
        &rename_map,
        render_rename_map(&baseline.linux_x86_64.dynamic_exports),
    )
    .expect("write rename map");

    build_internal_archive(&build_dir, &out_dir, &shared_link_contents, &rename_map, &internal_archive);

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=uv_internal");
    println!("cargo:rustc-cdylib-link-arg={soname_flag}");
    println!(
        "cargo:rustc-cdylib-link-arg=-Wl,--version-script={}",
        version_script.display()
    );
}

fn assert_supported_target() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_os != "linux" || target_arch != "x86_64" {
        panic!("this scaffold currently supports linux/x86_64 only");
    }
}

fn load_abi_baseline(path: &Path) -> AbiBaseline {
    let contents = fs::read_to_string(path).expect("abi baseline json");
    serde_json::from_str(&contents).expect("parse abi baseline json")
}

fn count_baseline_objects(link_txt: &str) -> usize {
    link_txt
        .split_whitespace()
        .filter(|token| token.ends_with(".o") || token.ends_with(".o\""))
        .count()
}

fn build_upstream(build_dir: &Path) {
    run(Command::new("cmake")
        .arg("--build")
        .arg(build_dir)
        .arg("--target")
        .arg("uv")
        .arg("uv_a"));
}

fn render_version_script(exports: &[String]) -> String {
    let mut map = String::from("Base {\n  global:\n");
    for symbol in exports {
        map.push_str("    ");
        map.push_str(symbol);
        map.push_str(";\n");
    }
    map.push_str("  local:\n    *;\n};\n");
    map
}

fn render_rename_map(exports: &[String]) -> String {
    let mut map = String::new();
    for symbol in exports {
        map.push_str(symbol);
        map.push(' ');
        map.push_str("uv_internal_");
        map.push_str(symbol);
        map.push('\n');
    }
    map
}

fn build_internal_archive(
    build_dir: &Path,
    out_dir: &Path,
    shared_link_txt: &str,
    rename_map: &Path,
    internal_archive: &Path,
) {
    let object_dir = out_dir.join("uv-internal-objects");
    if object_dir.exists() {
        fs::remove_dir_all(&object_dir).expect("remove stale internal object dir");
    }
    fs::create_dir_all(&object_dir).expect("create internal object dir");

    let mut objects = Vec::new();
    for rel in parse_object_paths(shared_link_txt) {
        let src = build_dir.join(&rel);
        let dst = object_dir.join(&rel);
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent).expect("create internal object subdir");
        }
        run(
            Command::new("objcopy")
                .arg(format!("--redefine-syms={}", rename_map.display()))
                .arg(&src)
                .arg(&dst),
        );
        objects.push(dst);
    }

    if internal_archive.exists() {
        fs::remove_file(internal_archive).expect("remove stale internal archive");
    }

    let mut ar = Command::new("ar");
    ar.arg("crs").arg(internal_archive);
    for object in &objects {
        ar.arg(object);
    }
    run(&mut ar);
}

fn parse_object_paths(link_txt: &str) -> Vec<PathBuf> {
    link_txt
        .split_whitespace()
        .map(|token| token.trim_matches('"'))
        .filter(|token| token.ends_with(".o"))
        .map(PathBuf::from)
        .collect()
}

fn run(command: &mut Command) {
    let status = command.status().unwrap_or_else(|err| {
        panic!("failed to execute {:?}: {err}", command);
    });
    if !status.success() {
        panic!("command {:?} exited with {status}", command);
    }
}
