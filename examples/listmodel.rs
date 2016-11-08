#![allow(non_snake_case)]
#[macro_use]
extern crate qml;

use qml::*;

fn main() {
    let mut qqae = QmlEngine::new();
    let mut qalm = QListModel::new(&["name", "number"]);
    qalm.append_row(qvarlist!["John", 42].into_iter());
    qalm.append_row(qvarlist!["Oak", 505].into_iter());
    qalm.insert_row(1, qvarlist!["Nick", 42].into_iter());
    qqae.set_property("listModel", &qalm.get_qvar());

    qqae.load_file("examples/listmodel.qml");
    qalm.set_data(vec![qvarlist!["OMG", 13317], qvarlist!["HACKED", 228]]);
    qqae.exec();
    qqae.quit();
}
