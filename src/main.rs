extern crate libc;

mod qmlengine;
use qmlengine::*;
fn main() {
    let qqae = QmlEngine::new();
    qqae.load("../../chart.qml");
    qqae.exec();
}
