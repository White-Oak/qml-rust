extern crate qml;

use qml::*;
fn main() {
    let mut qqae = QmlEngine::new();
    qqae.load_url("qrc:///qml/resources.qml");
    qqae.exec();
    qqae.quit();
}
