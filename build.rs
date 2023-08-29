/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */
#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use build_support::{get_acfutils_libs, get_target_platform, Platform};
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-env-changed=LIBACFUTILS");
    println!("cargo:rerun-if-env-changed=XPLANE_SDK");
    let acfutils_path = Path::new(env!("LIBACFUTILS"));
    let platform = get_target_platform();
    configure(platform, acfutils_path);

    #[cfg(feature = "generate-bindings")]
    generate_bindings(platform, acfutils_path);
}

fn configure(platform: Platform, acfutils_path: &Path) {
    println!(
        "cargo:rustc-link-search={}/{}/lib",
        acfutils_path.display(),
        platform.short()
    );

    for lib in get_acfutils_libs(platform) {
        println!("cargo:rustc-link-lib={lib}");
    }
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(platform: Platform, acfutils_path: &Path) {
    println!("cargo:rerun-if-changed=acfutils.h");
    let xplane_sdk_path = Path::new(env!("XPLANE_SDK"));
    bindgen::Builder::default()
        .header("acfutils.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(build_support::get_acfutils_cflags(
            platform,
            acfutils_path,
            xplane_sdk_path,
        ))
        .allowlist_file(".*/acfutils/conf.h")
        .allowlist_file(".*/acfutils/crc64.h")
        .allowlist_file(".*/acfutils/geom.h")
        .allowlist_file(".*/acfutils/log.h")
        // conf.h
        .blocklist_function("conf_get_lli.*")
        .blocklist_function("conf_set_lli.*")
        .blocklist_function("conf_read")
        .blocklist_function("conf_write")
        .blocklist_type("__sFILE")
        .blocklist_type("__sFILEX")
        .blocklist_type("FILE")
        .blocklist_type("fpos_t")
        // geom.h
        .blocklist_function("vect3l_.*")
        .blocklist_function("ecef2gl_l")
        .blocklist_function("gl2ecef_l")
        .blocklist_function("log_impl_v")
        .blocklist_type("vect3l_t")
        // general
        .blocklist_type("__builtin_va_list")
        .blocklist_type("__darwin_off_t")
        .blocklist_type("__darwin_va_list")
        .blocklist_type("__int64_t")
        .blocklist_type("__va_list_tag")
        .blocklist_type("va_list")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings");
}
