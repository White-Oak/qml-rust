#![allow(non_snake_case)]
// #![feature(trace_macros)]
#[macro_use]
extern crate qml;

use qml::*;

fn main() {
    let mut qqae = QmlEngine::new();
    // trace_macros!(true);
    let shortcut = qvarlist![["John", [2, 2]], ["Ivan", [10, 0]], ["Mary", [0, 1]]];
    qqae.set_and_store_property("values", shortcut);

    qqae.load_file("examples/qvarlists.qml");
    qqae.exec();
}
