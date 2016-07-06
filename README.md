# QML-rust - bindings for [Qt Quick](http://doc.qt.io/qt-5/qtquick-index.html)
Library is still in a rough shape.  
Bindings are based on [DOtherSide](https://github.com/filcuc/DOtherSide) C bindings for QML

# Examples
All examples are located in a folder [`examples/`](examples), under `example_name.rs` and `example_name.qml` names.

* `cargo run --example properties` for setting properties from Rust to QML.
* `cargo run --example listmodel` for an example of providing QML with list model from Rust.
* `cargo run --example sigslots` for an example of how to create your own `QObject` with signals and slots, and to communicate between QML and Rust. Also shows how to use `Q_OBJECT!` macro.
* `cargo run --example qvarlists` for an example of how to use `qvarlist!` macro to easily form `QVariant` (used to pass data to QML) of a complex array.

Requires CMake, Make, Qt (Core, Gui, Widgets, Quick) and, of course, Rust.

## In-app examples

* [Architect](https://github.com/White-Oak/architect/tree/qml-lib) - an app showing some git stats,
using qml-rust to provide properties and lists to QML in [here](https://github.com/White-Oak/architect/blob/qml-lib/src/view/qt.rs)
* [Kefia](https://github.com/White-Oak/kefia) - A simple package manager, that provides a QListModel to QML,
[here](https://github.com/White-Oak/kefia/blob/master/src/view.rs)

# Status
Done:
* Basic initialization and execution.
* Providing properties to QML files.
* QAbstractListModels - to provide changable models for QML items (early draft, still lacks proper mutability).
* QObjects: slots, signals (no properties for a whiel). Emitting signals and receiving slots works.

To be done:
* QML singletons
* etc
