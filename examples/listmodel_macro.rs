#[macro_use]
extern crate qml;

use qml::*;

Q_LISTMODEL!{
    pub QTestModel {
        name: &str,
        number: i32,
    }
}

fn main() {
    let mut qqae = QmlEngine::new();
    let mut qalm = QTestModel::new();
    qalm.insert_row("John", 42);
    qalm.insert_row("Oak", 505);
    // `&QTestModel` implements `Into<QVariant>`
    qqae.set_and_store_property("listModel", &qalm);

    qqae.load_file("examples/listmodel.qml");
    qalm.set_data(vec![("OMG", 13317), ("HACKED", 228)]);
    qqae.exec();
    qqae.quit();
}
