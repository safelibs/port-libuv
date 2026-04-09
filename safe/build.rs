use serde::Deserialize;
use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Deserialize)]
struct AbiBaseline {
    linux_x86_64: LinuxX8664Baseline,
}

#[derive(Debug, Deserialize)]
struct LinuxX8664Baseline {
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

    validate_link_contract(&baseline.linux_x86_64.soname, &shared_link_contents, &static_link_contents);

    build_upstream(&build_dir);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let profile_dir = cargo_profile_dir(&out_dir);
    stage_upstream_artifacts(&build_dir, &profile_dir);
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

fn validate_link_contract(soname: &str, shared_link_txt: &str, static_link_txt: &str) {
    let soname_flag = format!("-Wl,-soname,{soname}");
    if !shared_link_txt.contains(&soname_flag) {
        panic!("baseline shared linker contract no longer carries {soname_flag}");
    }

    for lib in ["pthread", "dl", "rt"] {
        let link_flag = format!("-l{lib}");
        if !shared_link_txt.contains(&link_flag) {
            panic!("baseline shared linker contract no longer carries {link_flag}");
        }
    }

    let static_object_count = count_baseline_objects(static_link_txt);
    if static_object_count == 0 {
        panic!("baseline static archive contract contains no object members");
    }
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

fn cargo_profile_dir(out_dir: &Path) -> PathBuf {
    out_dir
        .ancestors()
        .nth(3)
        .expect("target profile dir")
        .to_path_buf()
}

fn stage_upstream_artifacts(build_dir: &Path, profile_dir: &Path) {
    let shared_src = build_dir.join("libuv.so.1.0.0");
    let static_src = build_dir.join("libuv.a");
    let shared_dst = profile_dir.join("libuv.so.1.0.0");
    let static_dst = profile_dir.join("libuv.a");

    if !shared_src.is_file() || !static_src.is_file() {
        panic!(
            "upstream build did not produce expected artifacts in {}",
            build_dir.display()
        );
    }

    fs::create_dir_all(profile_dir).expect("create target profile dir");
    copy_file(&static_src, &static_dst);
    copy_file(&shared_src, &shared_dst);
    recreate_symlink(&profile_dir.join("libuv.so.1"), "libuv.so.1.0.0");
    recreate_symlink(&profile_dir.join("libuv.so"), "libuv.so.1");
}

fn copy_file(src: &Path, dst: &Path) {
    if dst.exists() {
        fs::remove_file(dst).expect("remove stale file");
    }
    fs::copy(src, dst).unwrap_or_else(|err| {
        panic!("copy {} -> {}: {err}", src.display(), dst.display());
    });
}

fn recreate_symlink(path: &Path, target: &str) {
    if path.exists() || path.is_symlink() {
        fs::remove_file(path).expect("remove stale symlink");
    }
    symlink(target, path).unwrap_or_else(|err| {
        panic!("symlink {} -> {}: {err}", path.display(), target);
    });
}

fn run(command: &mut Command) {
    let status = command.status().unwrap_or_else(|err| {
        panic!("failed to execute {:?}: {err}", command);
    });
    if !status.success() {
        panic!("command {:?} exited with {status}", command);
    }
}
