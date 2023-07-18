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
    let path = acfutils_redist_path.join("../pkg-config-deps");
    let output = std::process::Command::new(path)
        .args([get_arch(), "--libs"])
        .output()
        .expect("failed to run pkg-config-deps");
    let res = String::from_utf8(output.stdout).expect("Could not create string from stdout");
    res.split_whitespace().for_each(|s| {
        if s.starts_with("-L") {
            let s = &s[2..];
            println!("cargo:rustc-link-search={}", s);
        } else if s.starts_with("-l") {
            let s = &s[2..];
            println!("cargo:rustc-link-lib={s}");
        }
    });
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(acfutils_redist_path: &std::path::Path) {
    println!("cargo:rerun-if-changed=acfutils.h");

    let xplane_sdk_path = std::path::Path::new(env!("XPLANE_SDK"));
    bindgen::Builder::default()
        .header("acfutils.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args([
            &format!("-I{}/include", acfutils_redist_path.display()),
            &format!("-I{}/CHeaders/XPLM", xplane_sdk_path.display()),
            &format!("-D{}", get_xp_def()),
        ])
        .allowlist_file(allow(acfutils_redist_path, "crc64.h"))
        .allowlist_file(allow(acfutils_redist_path, "geom.h"))
        .allowlist_file(allow(acfutils_redist_path, "log.h"))
        .blocklist_function("vect3l_.*")
        .blocklist_function("ecef2gl_l")
        .blocklist_function("gl2ecef_l")
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

fn get_arch() -> &'static str {
    match get_target() {
        Target::Windows => "win-64",
        Target::MacOs => "mac-64",
        Target::Linux => "linux-64",
    }
}
