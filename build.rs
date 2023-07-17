/*
 * Copyright © 2023 David Dunwoody.
 *
 * All rights reserved.
 */

fn main() {
    println!("cargo:rerun-if-env-changed=XPLANE_SDK");
    println!("cargo:rerun-if-env-changed=LIBACFUTILS_REDIST");

    let acfutils_redist_path = std::path::Path::new(env!("LIBACFUTILS_REDIST"));

    configure(&acfutils_redist_path);

    #[cfg(feature = "generate-bindings")]
    generate_bindings(&acfutils_redist_path);

}

fn configure(acfutils_redist_path: &std::path::Path) {
    let dir = match get_target() {
        Target::Windows => "mingw64",
        Target::MacOs => "mac64",
        Target::Linux => "lin64",
    };

    println!("cargo:rustc-link-search={}/{dir}/lib", acfutils_redist_path.display());
    println!("cargo:rustc-link-lib=static=acfutils");
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(acfutils_redist_path: &std::path::Path) {
    println!("cargo:rerun-if-changed=acfutils.h");

    let xplane_sdk_path = std::path::Path::new(env!("XPLANE_SDK"));
    bindgen::Builder::default().header("acfutils.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args([
            &format!("-I{}/include", acfutils_redist_path.display()),
            &format!("-I{}/CHeaders/XPLM", xplane_sdk_path.display()),
            &format!("-D{}", get_xp_def()),
        ])
        .allowlist_file(allow(acfutils_redist_path, "crc64.h"))
        .allowlist_file(allow(acfutils_redist_path, "log.h"))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings");
}

#[cfg(feature = "generate-bindings")]
fn allow(acfutils_redist_path: &std::path::Path, file: &str) -> String {
    format!("{}/include/acfutils/{file}", acfutils_redist_path.display())
}

enum Target {
    Windows,
    MacOs,
    Linux,
}

fn get_target() -> Target {
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target == "macos" {
        Target::MacOs
    } else if target == "windows" {
        Target::Windows
    } else if target == "linux" {
        Target::Linux
    } else {
        panic!("Unsupported target: {target}");
    }
}

#[cfg(feature = "generate-bindings")]
fn get_xp_def() -> &'static str {
    match get_target() {
        Target::Windows => "IBM",
        Target::MacOs => "APL",
        Target::Linux => "LIN",
    }
}
