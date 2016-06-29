extern crate libc;

mod qmlengine;
mod qvariant;

pub use qmlengine::*;
pub use qvariant::*;
fn main() {
    let qqae = QmlEngine::new();
    qqae.load("../../chart.qml");
    qqae.exec();
    qqae.quit();
}
