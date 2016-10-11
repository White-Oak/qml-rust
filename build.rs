use std::env;
use std::process::Command;
use std::path::*;
use std::str::*;
use std::fs;

fn is_dylib() -> bool {
    option_env!("DyLib_DOtherSide").is_some()
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    fs::copy(Path::new(&manifest_dir).join("build_lib.sh"), Path::new(&out_dir).join("build_lib.sh")).unwrap();
    // Probably should unify this code
    let output = if is_dylib() {
        Command::new("sh")
            .current_dir(&out_dir)
            .arg("build_lib.sh")
            .env("IS_DYLIB", "1")
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e))
    } else {
        Command::new("sh")
            .current_dir(&out_dir)
            .arg("build_lib.sh")
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e))
    };

    if !output.status.success() {
        panic!("Cannot build qrc resource:\n{:#?}\n{:#?}",
               to_utf(&output.stdout),
               to_utf(&output.stderr));
    }

    let path = Path::new(&out_dir).join("DOtherSide").join("build").join("lib");

    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rerun-if-changed={}", path.display());
    if is_dylib() {
        println!("cargo:rustc-link-lib=dylib=DOtherSide");
    } else {
        println!("cargo:rustc-link-lib=static=DOtherSideStatic");
    }
    println!("cargo:rustc-link-lib=dylib=stdc++");

    let target = env::var("TARGET").expect("Environment variable TARGET not set");

    let osx_framework = if target.contains("darwin") { "=framework" }
                        else  { "" };
    // On Linux, libraries are named "Qt5Core", not "QtCore" as on OSX
    let linux_qt_lib_ver = if target.contains("linux") { "5" }
                           else  { "" };

    const QT_PLUGINS: [&'static str; 5] = ["Core", "Gui", "Qml", "Quick", "Widgets"];
    for plugin in &QT_PLUGINS {
        println!("cargo:rustc-link-lib{}=Qt{}{}", osx_framework, linux_qt_lib_ver, plugin);
    }
}

fn to_utf(buf: &[u8]) -> &str {
    match from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}
