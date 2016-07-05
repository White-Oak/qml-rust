extern crate qml;

use qml::*;
fn main() {
    let mut qqae = QmlEngine::new();
    let qvar: QVariant = 10.into();
    qqae.set_and_store_property("qVar1", 5);
    qqae.set_and_store_property("qVar2", 8.6);
    qqae.set_property("qVar3", &qvar);
    qqae.set_property("qVar4", &qvar);
    let vec: Vec<QVariant> = vec![42.into(), 43.into()];
    qqae.set_and_store_property("values", vec);
    qqae.load_file("examples/properties.qml");
    qqae.exec();
    qqae.quit();
}
