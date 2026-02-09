use std::env;
use std::path::PathBuf;

fn main() {
    // Determine library search path.
    // Priority: SYMENGINE_LIB_DIR env > relative path from project root.
    let lib_dir = if let Ok(dir) = env::var("SYMENGINE_LIB_DIR") {
        PathBuf::from(dir)
    } else {
        // Default: symengine.js/dist/wasm-unknown/lib from workspace root
        let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        manifest.join("../../symengine.js/dist/wasm-unknown/lib")
    };

    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    // Link order matters: symengine first, then C++ runtime, then C runtime
    println!("cargo:rustc-link-lib=static=symengine");

    // If built with GMP, also link libgmp
    if lib_dir.join("libgmp.a").exists() {
        println!("cargo:rustc-link-lib=static=gmp");
    }

    // C++ standard library (from wasi-sdk, shipped alongside libsymengine.a)
    if lib_dir.join("libc++.a").exists() {
        println!("cargo:rustc-link-lib=static=c++");
        println!("cargo:rustc-link-lib=static=c++abi");
    }

    // wasi-libc (provides malloc, free, printf, string ops, math, etc.)
    if lib_dir.join("libc.a").exists() {
        println!("cargo:rustc-link-lib=static=c");
    }

    // Compile WASI stubs and allocator bridge so the binary runs in
    // wasm32-unknown-unknown without a WASI runtime.
    let stubs = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("wasi_stub.c");
    if stubs.exists() {
        cc::Build::new()
            .file(&stubs)
            .target("wasm32-unknown-unknown")
            .opt_level(2)
            .compile("wasi_stub");
    }

    // Re-run if the library changes
    println!("cargo:rerun-if-env-changed=SYMENGINE_LIB_DIR");
    println!(
        "cargo:rerun-if-changed={}",
        lib_dir.join("libsymengine.a").display()
    );
    println!("cargo:rerun-if-changed=wasi_stub.c");
}
