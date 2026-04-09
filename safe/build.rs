use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    assert_supported_target();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let abi_baseline = manifest_dir.join("tools/abi-baseline.json");
    println!("cargo:rerun-if-changed={}", abi_baseline.display());

    let soname = load_soname(&abi_baseline);

    println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,{soname}");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=rt");
}

fn assert_supported_target() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_os != "linux" || target_arch != "x86_64" {
        panic!("safe/libuv currently supports linux/x86_64 only");
    }
}

fn load_soname(path: &Path) -> String {
    let contents = fs::read_to_string(path).expect("abi baseline json");
    for line in contents.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("\"soname\"") {
            continue;
        }

        let value = trimmed
            .split_once(':')
            .map(|(_, rest)| rest.trim())
            .and_then(|rest| rest.strip_prefix('"'))
            .and_then(|rest| rest.split('"').next())
            .filter(|value| !value.is_empty())
            .expect("parse soname from abi baseline");
        return value.to_owned();
    }

    panic!("abi baseline missing linux_x86_64.soname");
}
