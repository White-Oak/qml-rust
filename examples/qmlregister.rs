#![allow(non_snake_case)]
#[macro_use]
extern crate qml;

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

Q_OBJECT!(
pub Test as QTestSingleton{
    signals:
    slots:
    properties:
        temp: i32; read: get_temp, write: set_temp, notify: temp_changed;
});

Q_REGISTERABLE_QML!(QTest: Test as TestRsObject 1=>0, from TestModule);
Q_REGISTERABLE_QML!(QTestSingleton: Test as TestRsSingleton 1=>0, from TestModule);
fn main() {
    let mut qqae = QmlEngine::new();
    Q_REGISTER_QML!(QTest);
    Q_REGISTER_SINGLETON_QML!(QTestSingleton);
    qqae.load_file("examples/qmlregister.qml");
    qqae.exec();

    qqae.quit();
}
