#[macro_use]
extern crate qml;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use qml::*;
fn main() {
    let qqae = QmlEngine::new();
    let mut qalm = QListModel::new(&["name", "number"]);
    qalm.insert_row(qvarlist!["John", 42].into_iter());
    qalm.insert_row(qvarlist!["Oak", 505].into_iter());
    let qvar = qalm.get_qvar();
    qqae.set_property("listModel", &qvar);


    qqae.load_file("examples/listmodel.qml");
    thread::spawn(move || {
        qqae.exec();
        qqae.quit();
    });
    thread::sleep(Duration::from_secs(2));
    qalm.set_data(vec![qvarlist!["OMG", 13317], qvarlist!["HACKED", 228]]);
}
