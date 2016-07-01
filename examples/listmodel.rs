extern crate qml;

use qml::*;
fn main() {
    let qqae = QmlEngine::new();
    let roles = ["name", "number"];
    let smth = ["John".into(), 42.into()];
    let mut qalm = QListModel::new(&roles);
    qalm.insert_row(&smth);
    let qvar = qalm.get_qvar();
    qqae.set_property("listModel", &qvar);
    qqae.load_file("examples/listmodel.qml");
    qqae.exec();
    qqae.quit();
}
// 140016251736832 140541045150464


// PUSHED at 1 and 0x7fed1ecc1120
// ROW COUNT GOT at 2097865012304223517 and 0x7fed1ecc1240
