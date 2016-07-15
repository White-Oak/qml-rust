#[macro_use]
extern crate qml;
use std::thread;
use std::sync::*;
use std::time::Duration;

use qml::*;

#[derive(Default)]
pub struct Test;

Q_OBJECT!(
pub Test as QTest{
    signals:
    slots:
    properties:
});

Q_REGISTERABLE_QML!(QTest: Test as TestRsObject 1=>0, from TestModule);
fn main() {
    let mut qqae = QmlEngine::new();
    Q_REGISTER_QML!(QTest);
    qqae.load_file("examples/qmlregister.qml");
    qqae.exec();

    qqae.quit();
}
