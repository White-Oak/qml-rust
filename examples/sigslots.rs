#![allow(non_snake_case)]
#[macro_use]
extern crate qml;

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
        list: QVariantList; read: get_list, write: set_list, notify: list_changed;
});

impl QTest {
    pub fn click(&self) -> Option<&QVariant> {
        println!("IT CLICKED");
        self.updateText("Woah, Rust has noticed you".into());
        None
    }
}

fn main() {
    let mut qqae = QmlEngine::new();
    let mut qtest = QTest::new(Test, "OAK".into(), qvarlist![2, 4, 5].into());
    qtest.set_name("Swapped".into());
    // assert_eq!(qtest.get_name(), "Swapped".to_string());
    qqae.set_and_store_property("test", qtest.get_qobj());
    qqae.load_file("examples/sigslots.qml");
    qqae.exec();

    qqae.quit();
}
