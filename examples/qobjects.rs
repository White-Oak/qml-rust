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
 fn testname (a: i32, b: i32, f: f32, d: f64);
    slots:
 fn launchGoose(i: i32, launchText: String);
    properties:
});

fn main() {
    let mut qtest = QTest::new(Test);
    qtest.testname(54, 55, 5_f32, 6_f64);
    qtest.qslot_call("launchGoose",
                     vec![42.into(), "QML Rust".to_string().into()]);
    println!("{:?}", qtest.qmeta());
}
