use super::super::*;

#[test]
fn new() {
    let _ = QmlEngine::new();
}

#[test]
fn load_file() {
    let mut qqae = QmlEngine::new();
    qqae.load_file("examples/listmodel.qml");
}

#[test]
fn load_data() {
    let mut qqae = QmlEngine::new();
    qqae.load_data(include_str!("../../examples/listmodel.qml"));
}

#[test]
fn load_url_file() {
    let mut qqae = QmlEngine::new();
    let path_raw = ::std::env::current_dir()
        .unwrap()
        .join("examples")
        .join("listmodel.qml");
    let path = if cfg!(windows) {
        format!("file:///{}", path_raw.display())
    } else {
        format!("file://{}", path_raw.display())
    };
    qqae.load_url(&path);
}
