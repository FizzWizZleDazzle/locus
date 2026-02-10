use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.starts_with("wasm32") {
        build_wasm();
    } else {
        build_native();
    }
}

/// WASM target: link from symengine.js prebuilt libs, compile wasi stubs.
fn build_wasm() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let lib_dir = if let Ok(dir) = env::var("SYMENGINE_LIB_DIR") {
        PathBuf::from(dir)
    } else {
        manifest.join("../../symengine.js/dist/wasm-unknown/lib")
    };

    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    println!("cargo:rustc-link-lib=static=symengine");

    if lib_dir.join("libgmp.a").exists() {
        println!("cargo:rustc-link-lib=static=gmp");
    }

    if lib_dir.join("libc++.a").exists() {
        println!("cargo:rustc-link-lib=static=c++");
        println!("cargo:rustc-link-lib=static=c++abi");
    }

    if lib_dir.join("libc.a").exists() {
        println!("cargo:rustc-link-lib=static=c");
    }

    // Compile WASI stubs for wasm32-unknown-unknown
    let stubs = manifest.join("wasi_stub.c");
    if stubs.exists() {
        cc::Build::new()
            .file(&stubs)
            .target("wasm32-unknown-unknown")
            .opt_level(2)
            .compile("wasi_stub");
    }

    println!("cargo:rerun-if-env-changed=SYMENGINE_LIB_DIR");
    println!(
        "cargo:rerun-if-changed={}",
        lib_dir.join("libsymengine.a").display()
    );
    println!("cargo:rerun-if-changed=wasi_stub.c");
}

/// Native target: link system-installed SymEngine + GMP.
fn build_native() {
    // Static link libsymengine
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=static=symengine");

    // Dynamic link system GMP and C++ runtime
    println!("cargo:rustc-link-lib=dylib=gmp");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    println!("cargo:rerun-if-changed=/usr/local/lib/libsymengine.a");
}
