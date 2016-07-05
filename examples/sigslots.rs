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
});

impl QTest {
    pub fn click(&self) {
        println!("IT CLICKED");
        self.updateText("Woah, Rust has noticed you".into());
    }
}

fn main() {
    let mut qqae = QmlEngine::new();
    let mut qtest = QTest::new(Test);
    qqae.set_and_store_property("test", qtest.get_qobj());
    qqae.load_file("examples/sigslots.qml");
    qqae.exec();

    qqae.quit();
}
