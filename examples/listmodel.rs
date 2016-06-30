extern crate qml;

use qml::*;
fn main() {
    let qqae = QmlEngine::new();
    let qalm = RUN_QALM();
    let qvar = qalm.get_qvar();
    qqae.set_property("listModel", &qvar);
    qqae.load("../../../examples/listmodel.qml");
    qqae.exec();
    qqae.quit();
}
