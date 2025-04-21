use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_else(|_| String::new());
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| String::new());

    if target.contains("windows") {
        // On Windows, the DLL will be found at runtime.
        println!("cargo:rustc-link-lib=dylib=uEye_api");
        println!("cargo:rerun-if-changed=build.rs");
    } else if target.contains("linux") {
        if !arch.contains("x86") {
            panic!("Unsupported Linux architecture: only x86 and x86_64 are supported");
        }

        println!("cargo:rustc-link-search=native=/usr/lib");
        println!("cargo:rustc-link-lib=dylib=ueye_api");
        println!("cargo:rerun-if-changed=build.rs");
    } else {
        panic!("Unsupported platform: only Windows and Linux are supported");
    }
}
