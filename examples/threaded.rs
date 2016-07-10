#[macro_use]
extern crate qml;

use std::thread;
use std::time::Duration;
use qml::*;

pub struct Logic;

impl QLogic {
    pub fn downloadPage(&mut self, url: String) {
        self.threaded(|s| {
            thread::sleep(Duration::from_secs(2));;
            s.pageDownloaded(url);
        })
    }
}

Q_OBJECT!{
pub Logic as QLogic {
    signals:
        fn pageDownloaded(response: String);
    slots:
        fn downloadPage(url: String);
    properties:
}
}

fn main() {
    let mut qqae = QmlEngine::new();
    let qlogic = QLogic::new(Logic);
    qqae.set_and_store_property("logic", qlogic.get_qobj());
    qqae.load_file("examples/threaded.qml");
    qqae.exec();
}
