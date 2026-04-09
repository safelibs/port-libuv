use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

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
    let shared_link_txt = repo_root.join("original/build-checker/CMakeFiles/uv.dir/link.txt");
    let static_link_txt = repo_root.join("original/build-checker/CMakeFiles/uv_a.dir/link.txt");
    let abi_baseline = manifest_dir.join("tools/abi-baseline.json");

    for path in [&shared_link_txt, &static_link_txt, &abi_baseline] {
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

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let version_script = out_dir.join("libuv.map");
    fs::write(
        &version_script,
        render_version_script(&baseline.linux_x86_64.dynamic_exports),
    )
    .expect("write version script");

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
