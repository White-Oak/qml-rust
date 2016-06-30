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


#[test]
fn properties() {
    let qqae = QmlEngine::new();
    let qvar: QVariant = 10.into();
    qqae.set_and_store_property("qVar1", 5);
    qqae.set_and_store_property("qVar2", 8.6);
    qqae.set_property("qVar3", &qvar);
    qqae.set_property("qVar4", &qvar);
    qqae.load("../../examples/properties.qml");
    qqae.exec();
    qqae.quit();
}
