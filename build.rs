extern crate pkg_config;
use std::env;
use std::process::Command;
use std::path::*;

fn main() {
    Command::new("sh")
        .arg("build_lib.sh")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&manifest_dir).join("DOtherSide").join("build").join("lib");

    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rerun-if-changed={}", path.display());
    println!("cargo:rustc-link-lib=static=DOtherSideStatic");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    pkg_config::probe_library("Qt5Core Qt5Gui Qt5Qml Qt5Quick Qt5Widgets").unwrap();
}
