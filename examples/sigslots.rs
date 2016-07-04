#[macro_use]
extern crate qml;

use qml::*;

struct Test;

impl Test {
    pub fn click(&self) {
        println!("IT CLICKED");
        self.updateText("Woah".into());
    }
}

Q_OBJECT!(
Test:
    signals:
        fn updateText(s: String);
    slots:
         fn click();
);

fn main() {
    let mut test = Test;
    let mut qqae = QmlEngine::new();
    let qobj = QObject::new(&mut test);
    qqae.set_and_store_property("test", qobj);
    qqae.load_file("examples/sigslots.qml");
    qqae.exec();
}
