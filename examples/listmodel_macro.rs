#![allow(non_snake_case)]
#[macro_use]
extern crate qml;

use qml::*;

Q_LISTMODEL!{
    pub QTestModel {
        name: String,
        number: i32,
    }
}

fn main() {
    let mut qqae = QmlEngine::new();
    let mut qalm = QTestModel::new();
    qalm.insert_row("John".into(), 42);
    qalm.insert_row("Oak".into(), 505);
    // `&QTestModel` implements `Into<QVariant>`
    qqae.set_and_store_property("listModel", qalm.get_qvar());

    qqae.load_file("examples/listmodel.qml");
    qalm.set_data(vec![("OMG".into(), 13317), ("HACKED".into(), 228)]);
    qqae.exec();
    qqae.quit();
}
