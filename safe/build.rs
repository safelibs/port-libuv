use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use quote::quote;
use syn::{parse_file, parse_quote, FnArg, ForeignItem, ForeignItemFn, Item, Pat};

const VARIADIC_EXPORTS: &[&str] = &["uv_loop_configure"];

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing OUT_DIR"));
    let include_dir = manifest_dir.join("include");
    let header = include_dir.join("uv.h");
    let version_script = manifest_dir.join("abi/libuv.map");
    let legacy_manifest = manifest_dir.join("legacy/linux-manifest.toml");
    let exported_symbols_path = manifest_dir.join("abi/linux-exported-symbols.txt");
    let rename_header = manifest_dir.join("legacy/generated/legacy_rename.h");
    let bindings_path = out_dir.join("bindings.rs");
    let legacy_bindings_path = out_dir.join("legacy_bindings.rs");
    let ffi_exports_path = out_dir.join("ffi_exports_generated.rs");
    let build_manifest_path = out_dir.join("libuv-build-manifest.json");
    let legacy_sources = read_legacy_sources(&legacy_manifest);
    let exported_symbols = read_sorted_lines(&exported_symbols_path);

    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("build.rs").display()
    );
    println!("cargo:rerun-if-changed={}", header.display());
    println!("cargo:rerun-if-changed={}", include_dir.display());
    println!("cargo:rerun-if-changed={}", version_script.display());
    println!("cargo:rerun-if-changed={}", legacy_manifest.display());
    println!("cargo:rerun-if-changed={}", exported_symbols_path.display());
    println!("cargo:rerun-if-changed={}", rename_header.display());
    for source in &legacy_sources {
        println!("cargo:rerun-if-changed={source}");
    }

    emit_linux_link_args(&version_script);
    generate_bindings(&header, &include_dir, &bindings_path);

    let uv_functions = parse_uv_functions(&bindings_path);
    generate_legacy_bindings(&legacy_bindings_path, &exported_symbols, &uv_functions);
    generate_ffi_exports(&ffi_exports_path, &exported_symbols, &uv_functions);

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let mut production_non_rust_sources = Vec::new();
    let generated_production_non_rust_sources: Vec<String> = Vec::new();
    if target_os == "linux" {
        compile_legacy_sources(&manifest_dir, &legacy_sources, &rename_header);
        emit_linux_native_link_libs();
        production_non_rust_sources = legacy_sources.clone();
    }

    write_build_manifest(
        &build_manifest_path,
        &legacy_sources,
        &production_non_rust_sources,
        &generated_production_non_rust_sources,
    );
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

fn emit_linux_native_link_libs() {
    for lib in ["pthread", "dl", "rt"] {
        println!("cargo:rustc-link-lib={lib}");
    }
}

fn generate_bindings(header: &Path, include_dir: &Path, bindings_path: &Path) {
    let bindings = bindgen::Builder::default()
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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate libuv bindings");

    bindings
        .write_to_file(bindings_path)
        .expect("failed to write generated bindings");
}

fn parse_uv_functions(bindings_path: &Path) -> BTreeMap<String, ForeignItemFn> {
    let syntax =
        parse_file(&fs::read_to_string(bindings_path).expect("failed to read generated bindings"))
            .expect("failed to parse generated bindings");

    let mut functions = BTreeMap::new();
    for item in syntax.items {
        let Item::ForeignMod(foreign_mod) = item else {
            continue;
        };
        for foreign_item in foreign_mod.items {
            let ForeignItem::Fn(function) = foreign_item else {
                continue;
            };
            let name = function.sig.ident.to_string();
            if name.starts_with("uv_") {
                functions.insert(name, function);
            }
        }
    }
    functions
}

fn generate_legacy_bindings(
    output_path: &Path,
    exported_symbols: &[String],
    uv_functions: &BTreeMap<String, ForeignItemFn>,
) {
    let mut declarations = Vec::with_capacity(exported_symbols.len());

    for symbol in exported_symbols {
        if VARIADIC_EXPORTS.contains(&symbol.as_str()) {
            continue;
        }

        let mut function = uv_functions
            .get(symbol)
            .unwrap_or_else(|| panic!("missing binding declaration for {symbol}"))
            .clone();
        let link_name = syn::LitStr::new(
            &format!("uv_legacy_{symbol}"),
            proc_macro2::Span::call_site(),
        );
        function.attrs = vec![parse_quote!(#[link_name = #link_name])];
        declarations.push(function);
    }

    let tokens = quote! {
        unsafe extern "C" {
            #(#declarations)*
        }
    };

    fs::write(output_path, tokens.to_string()).expect("failed to write legacy bindings");
}

fn generate_ffi_exports(
    output_path: &Path,
    exported_symbols: &[String],
    uv_functions: &BTreeMap<String, ForeignItemFn>,
) {
    let mut wrappers = Vec::new();

    for symbol in exported_symbols {
        if VARIADIC_EXPORTS.contains(&symbol.as_str()) {
            continue;
        }

        let function = uv_functions
            .get(symbol)
            .unwrap_or_else(|| panic!("missing binding declaration for {symbol}"));
        if function.sig.variadic.is_some() {
            panic!("variadic export {symbol} is not handled by Rust code generation");
        }

        let ident = &function.sig.ident;
        let inputs = &function.sig.inputs;
        let output = &function.sig.output;
        let args = function
            .sig
            .inputs
            .iter()
            .map(argument_pattern_ident)
            .collect::<Vec<_>>();

        wrappers.push(quote! {
            #[no_mangle]
            pub unsafe extern "C" fn #ident(#inputs) #output {
                crate::legacy::#ident(#(#args),*)
            }
        });
    }

    let tokens = quote! {
        #(#wrappers)*
    };

    fs::write(output_path, tokens.to_string()).expect("failed to write ffi exports");
}

fn argument_pattern_ident(argument: &FnArg) -> syn::Ident {
    let FnArg::Typed(pattern) = argument else {
        panic!("unexpected receiver in generated binding");
    };

    match pattern.pat.as_ref() {
        Pat::Ident(ident) => ident.ident.clone(),
        other => panic!(
            "unsupported argument pattern in generated binding: {}",
            quote! { #other }
        ),
    }
}

fn compile_legacy_sources(manifest_dir: &Path, legacy_sources: &[String], rename_header: &Path) {
    let mut build = cc::Build::new();
    build
        .cargo_metadata(true)
        .pic(true)
        .warnings(false)
        .flag_if_supported("-fno-strict-aliasing")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wstrict-prototypes")
        .flag_if_supported("-Wextra")
        .define("_GNU_SOURCE", Some("1"))
        .define("_POSIX_C_SOURCE", Some("200112"))
        .define("_FILE_OFFSET_BITS", Some("64"))
        .define("_LARGEFILE_SOURCE", None)
        .flag("-include")
        .flag(
            rename_header
                .to_str()
                .expect("rename header path must be valid UTF-8"),
        )
        .include(manifest_dir.join("include"))
        .include(manifest_dir.join("../original/include"))
        .include(manifest_dir.join("../original/src"));

    for source in legacy_sources {
        build.file(source);
    }

    build.compile("uv_legacy");
}

fn write_build_manifest(
    output_path: &Path,
    legacy_manifest_sources: &[String],
    production_non_rust_sources: &[String],
    generated_production_non_rust_sources: &[String],
) {
    let json = format!(
        concat!(
            "{{\n",
            "  \"generated_production_non_rust_sources\": {},\n",
            "  \"legacy_manifest_sources\": {},\n",
            "  \"production_non_rust_sources\": {}\n",
            "}}\n"
        ),
        json_array(generated_production_non_rust_sources),
        json_array(legacy_manifest_sources),
        json_array(production_non_rust_sources),
    );

    fs::write(output_path, json).expect("failed to write libuv build manifest");
}

fn json_array(values: &[String]) -> String {
    let body = values
        .iter()
        .map(|value| format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{body}]")
}

fn read_sorted_lines(path: &Path) -> Vec<String> {
    let mut lines = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    if lines.windows(2).any(|window| window[0] >= window[1]) {
        lines.sort();
        lines.dedup();
        panic!("{} must be sorted and unique", path.display());
    }

    lines
}

fn read_legacy_sources(path: &Path) -> Vec<String> {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

    let mut sources = Vec::new();
    for line in contents.lines() {
        let trimmed = line.trim();
        if let Some(stripped) = trimmed
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix("\","))
        {
            sources.push(stripped.to_string());
            continue;
        }
        if let Some(stripped) = trimmed
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
        {
            sources.push(stripped.to_string());
        }
    }

    if sources.windows(2).any(|window| window[0] >= window[1]) {
        panic!(
            "{} must contain sorted, unique source paths",
            path.display()
        );
    }
    if sources
        .iter()
        .any(|source| !Path::new(source).is_absolute())
    {
        panic!("{} must contain absolute source paths", path.display());
    }

    sources
}
