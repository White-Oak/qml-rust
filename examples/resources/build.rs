use std::env;
use std::process::Command;
use std::fs;
use std::path::*;


fn main() {
    Command::new("sh")
        .arg("build_lib.sh")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&manifest_dir).join("resources");

    fs::copy(path.join("lib.a"), path.join("libqrc.a")).unwrap();

    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-lib=static=qrc");
}
