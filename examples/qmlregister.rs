#[macro_use]
extern crate qml;
use std::thread;
use std::sync::*;
use std::time::Duration;

use qml::*;

#[derive(Default)]
pub struct Test;

impl QTest {
    fn assure_everything_okay(&mut self) -> Option<&QVariant> {
        println!("It's okay");
        None
    }
}

Q_OBJECT!(
pub Test as QTest{
    signals:
    slots:
        fn assure_everything_okay();
    properties:
        name: String; read: get_name, write: set_name, notify: name_changed;
});

Q_REGISTERABLE_QML!(QTest: Test as TestRsObject 1=>0, from TestModule);
fn main() {
    let mut qqae = QmlEngine::new();
    Q_REGISTER_QML!(QTest);
    qqae.load_file("examples/qmlregister.qml");
    qqae.exec();

    qqae.quit();
}
