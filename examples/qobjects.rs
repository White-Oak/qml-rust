#![allow(non_snake_case)]
#[macro_use]
extern crate qml;

use qml::*;

pub struct Test;

impl Test {
    pub fn launchGoose(&self, i: i32, i2: String) -> Option<&QVariant> {
        println!("GOOSE HI from {} and {}", i2, i);
        None
    }
}

Q_OBJECT!(
pub Test as QTest{
    signals:
        fn testname (a: i32, b: i32, f: f32, d: f64, list: QVariantList);
    slots:
        fn launchGoose(i: i32, launchText: String);
    properties:
        name: String; read: get_name, write: set_name, notify: name_changed;
        i: i32; read: get_i, write: set_i, notify: i_changed;
        f: f32; read: get_f, write: set_f, notify: f_changed;
        d: f64; read: get_d, write: set_d, notify: d_changed;
        list: QVariantList; read: get_list, write: set_list, notify: list_changed;
});

fn main() {
    let mut qtest = QTest::new(Test, "name".into(), 5, 5_f32, 5_f64, qvarlist![["John", [2, 2]], ["Ivan", [10, 0]], ["Mary", [0, 1]]].into());
    qtest.testname(54, 55, 5_f32, 6_f64, qvarlist![["John", [2, 2]], ["Ivan", [10, 0]], ["Mary", [0, 1]]].into());
    qtest.qslot_call("launchGoose",
                     vec![42.into(), "QML Rust".to_string().into()]);
    println!("{:?}", qtest.qmeta());
}
