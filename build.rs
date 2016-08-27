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

    let target = env::var("TARGET").expect("Environment variable TARGET not set");

    let osx_framework = if target.contains("darwin") { "=framework" }
                        else  { "" };
    // On Linux, libraries are name "Qt5Core", not "QtCore" as on OSX
    let linux_qt_lib_ver = if target.contains("linux") { "5" }
                           else  { "" };

    println!("cargo:rustc-link-lib{}=Qt{}Core", osx_framework, linux_qt_lib_ver);
    println!("cargo:rustc-link-lib{}=Qt{}Gui", osx_framework, linux_qt_lib_ver);
    println!("cargo:rustc-link-lib{}=Qt{}Qml", osx_framework, linux_qt_lib_ver);
    println!("cargo:rustc-link-lib{}=Qt{}Quick", osx_framework, linux_qt_lib_ver);
    println!("cargo:rustc-link-lib{}=Qt{}Widgets", osx_framework, linux_qt_lib_ver);
}
