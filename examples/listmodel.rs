extern crate qml;

use qml::*;
fn main() {
    let qqae = QmlEngine::new();
    let mut qalm = QListModel::new(&["name", "number"]);
    let row: Vec<QVariant> = vec!["John".into(), 42.into()];
    qalm.insert_row(row.into_iter());
    let row: Vec<QVariant> = vec!["Oak".into(), 505.into()];
    qalm.insert_row(row.into_iter());
    let qvar = qalm.get_qvar();
    qqae.set_property("listModel", &qvar);
    qqae.load_file("examples/listmodel.qml");
    qqae.exec();
    qqae.quit();
}
