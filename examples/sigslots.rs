#[macro_use]
extern crate qml;
use std::thread;
use std::sync::*;
use std::time::Duration;

use qml::*;

pub struct Test;

Q_OBJECT!(
pub Test as QTest{
    signals:
        fn updateText(s: String);
    slots:
        fn click();
    properties:
        name: String; read: get_name, write: set_name, notify: name_changed;
});

impl QTest {
    pub fn click(&self) {
        println!("IT CLICKED");
        self.updateText("Woah, Rust has noticed you".into());
    }
}

fn main() {
    let mut qqae = QmlEngine::new();
    let mut qtest = QTest::new(Test, "OAK".into());
    qtest.set_name("Swapped".into());
    assert_eq!(qtest.get_name(), "Swapped".to_string());
    qqae.set_and_store_property("test", qtest.get_qobj());
    qqae.load_file("examples/sigslots.qml");
    qqae.exec();

    qqae.quit();
}
