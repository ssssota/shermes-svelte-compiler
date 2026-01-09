use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Build svelte.o with shermes
    let status = Command::new("shermes")
        .args(["-O", "-c", "-exported-unit=svelte"])
        .arg(format!("-o={}", out_dir.join("svelte.o").display()))
        .arg(manifest_dir.join("dist/index.cjs"))
        .status()
        .expect("Failed to run shermes");
    assert!(status.success(), "shermes failed");

    // Build compiler.o with clang++
    cc::Build::new()
        .cpp(true)
        .compiler("clang++")
        .file("src/compiler.cpp")
        .flag("-std=c++17")
        .flag("-stdlib=libstdc++")
        .flag("-O3")
        .include("/hermes/hermes/API")
        .include("/hermes/hermes/API/jsi")
        .include("/hermes/hermes/include")
        .include("/hermes/hermes/public")
        .include("/hermes/build/lib/config")
        .compile("compiler");

    // Link svelte.o
    println!(
        "cargo:rustc-link-arg={}",
        out_dir.join("svelte.o").display()
    );

    // Library search paths
    println!("cargo:rustc-link-search=/hermes/build/lib");
    println!("cargo:rustc-link-search=/hermes/build/jsi");
    println!("cargo:rustc-link-search=/hermes/build/tools/shermes");
    println!("cargo:rustc-link-search=/hermes/build/external/boost/boost_1_86_0/libs/context/");

    // Link libraries (order matters!)
    println!("cargo:rustc-link-lib=static=shermes_console_a");
    println!("cargo:rustc-link-lib=static=hermesvm_a");
    println!("cargo:rustc-link-lib=static=jsi");
    println!("cargo:rustc-link-lib=static=boost_context");
    println!("cargo:rustc-link-lib=icuuc");
    println!("cargo:rustc-link-lib=icui18n");
    println!("cargo:rustc-link-lib=atomic");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=gcc");

    // Re-run build.rs if source files change
    println!("cargo:rerun-if-changed=src/compiler.cpp");
    println!("cargo:rerun-if-changed=dist/index.cjs");
}
