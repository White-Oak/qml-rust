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

#[test]
fn load_url_qrc() {
    use std::process::Command;
    let path_raw = ::std::env::current_dir()
        .unwrap()
        .join("examples")
        .join("resources");
    Command::new("cargo")
        .arg("run")
        .current_dir(path_raw)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
}

#[test]
fn event_loop() {
    let mut engine = QmlEngine::new();
    engine.load_file("examples/listmodel.qml");

    for _ in 0..100 {
        engine.send_posted_events();
        engine.process_events();
        print!(".");
    }
    println!("");
}
