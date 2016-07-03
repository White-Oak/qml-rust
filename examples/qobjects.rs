#[macro_use]
extern crate qml;

use qml::*;

struct Test;

impl Test {
    pub fn launchGoose(&self, i: i32, i2: i32) {
        println!("GOOSE PRIVET {} and {}", i, i2);
    }
}

Q_OBJECT!(
Test:
    signals:
         fn testname (a: i32, b: i32);

    slots:
         fn launchGoose(i: i32, i2: i32);
);

fn main() {
    let mut test = Test;
    test.testname(54, 55);
    test.qmeta_slots("launchGoose", vec![5.into(), 6.into()]);
}
