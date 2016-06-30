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

    // use std::io::prelude::*;
    // use std::fs::File;
    //
    // let mut f = File::create("foo.txt").unwrap();
    // f.write_all(format!("{}", path.display()).as_bytes()).unwrap();
    println!("cargo:rustc-flags=-L {}", path.display());
    println!("cargo:rerun-if-changed={}", path.display());
    println!("cargo:rustc-link-lib=dylib=DOtherSide");
}
