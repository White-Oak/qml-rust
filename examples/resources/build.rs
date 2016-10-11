extern crate fs_utils;

use std::env;
use std::process::Command;
use std::path::*;
use std::str::*;

use fs_utils::*;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&manifest_dir).join("resources");
    let path_res = copy::copy_directory(path, &out_dir);
    let path = match path_res {
        Ok(path) => path,
        Err(err) => {
            match err {
                copy::Error::DestinationDirectoryExists(dir) => dir,
                _ => panic!("failed to copy resources dir: {}", err),
            }
        }
    };

    let output = Command::new("sh")
        .current_dir(&path)
        .arg("build_lib.sh")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if !output.status.success() {
        panic!("Cannot build qrc resource:\n{:#?}\n{:#?}",
               to_utf(&output.stdout),
               to_utf(&output.stderr));
    }

    println!("cargo:rerun-if-changed={}", path.display());
    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-lib=static=qrc");
}

fn to_utf(buf: &[u8]) -> &str {
    match from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}
