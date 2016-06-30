extern crate libc;

mod qmlengine;
mod qvariant;

pub use qmlengine::*;
pub use qvariant::*;
fn main() {
    let qqae = QmlEngine::new();
    let qvar: QVariant = 5.into();
    qqae.set_property("qVar1", &qvar);
    qqae.set_property("qVar2", &qvar);
    qqae.set_property("qVar3", &qvar);
    qqae.set_property("qVar4", &qvar);
    qqae.load("../../examples/properties.qml");
    qqae.exec();
    qqae.quit();
}
