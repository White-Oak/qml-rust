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
    let mut test = Box::new(Test);
    println!("{:?} start", test.as_ref() as *const Test);
    let mut qqae = QmlEngine::new();
    let wrap = test.singleton();
    let guard = wrap.inner.lock().unwrap();
    qqae.set_and_store_property("test", &guard as &QObject);
    qqae.load_file("examples/sigslots.qml");
    println!("{:?}", test.qmeta());
    qqae.exec();
}
