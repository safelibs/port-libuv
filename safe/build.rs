use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let include_dir = manifest_dir.join("include");
    let header = include_dir.join("uv.h");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing OUT_DIR"));
    let version_script = manifest_dir.join("abi/libuv.map");
    let legacy_manifest = manifest_dir.join("legacy/linux-manifest.toml");

    println!("cargo:rerun-if-changed={}", header.display());
    println!("cargo:rerun-if-changed={}", version_script.display());
    println!("cargo:rerun-if-changed={}", legacy_manifest.display());
    println!("cargo:rerun-if-changed={}", include_dir.display());
    println!("cargo:rerun-if-changed={}", manifest_dir.join("abi/linux-exported-symbols.txt").display());

    emit_linux_link_args(&version_script);
    generate_bindings(&header, &include_dir, &out_dir);

    let _legacy_sources = read_legacy_sources(&legacy_manifest);
}

fn emit_linux_link_args(version_script: &Path) {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("linux") {
        return;
    }

    println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libuv.so.1");
    println!(
        "cargo:rustc-cdylib-link-arg=-Wl,--version-script={}",
        version_script.display()
    );
}

fn generate_bindings(header: &Path, include_dir: &Path, out_dir: &Path) {
    let bindings_path = out_dir.join("bindings.rs");
    let builder = bindgen::Builder::default()
        .header(header.display().to_string())
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg("-D_GNU_SOURCE")
        .clang_arg("-D_LARGEFILE_SOURCE")
        .clang_arg("-D_FILE_OFFSET_BITS=64")
        .allowlist_function("^uv[_A-Za-z0-9]*$")
        .allowlist_type("^uv[_A-Za-z0-9]*$")
        .allowlist_var("^UV[_A-Za-z0-9]*$")
        .ctypes_prefix("libc")
        .generate_comments(false)
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    match builder.generate() {
        Ok(bindings) => bindings
            .write_to_file(&bindings_path)
            .expect("failed to write generated bindings"),
        Err(error) => {
            let fallback = format!(
                "// bindgen fallback generated during scaffold bootstrap\n// {error}\n"
            );
            fs::write(&bindings_path, fallback).expect("failed to write fallback bindings");
            println!("cargo:warning=bindgen could not parse uv.h during scaffold setup: {error}");
        }
    }
}

fn read_legacy_sources(path: &Path) -> Vec<String> {
    let Ok(contents) = fs::read_to_string(path) else {
        return Vec::new();
    };

    let mut sources = Vec::new();
    for line in contents.lines() {
        let trimmed = line.trim();
        if let Some(stripped) = trimmed.strip_prefix('"').and_then(|value| value.strip_suffix("\",")) {
            sources.push(stripped.to_string());
            continue;
        }
        if let Some(stripped) = trimmed.strip_prefix('"').and_then(|value| value.strip_suffix('"')) {
            sources.push(stripped.to_string());
        }
    }
    sources
}
