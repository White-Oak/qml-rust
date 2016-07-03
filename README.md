# QML-rust - bindings for [Qt Quick](http://doc.qt.io/qt-5/qtquick-index.html)
Library is still in a rough shape.  
Bindings are based on [DOtherSide](https://github.com/filcuc/DOtherSide) C bindings for QML

# Examples
* `cargo run --example properties` for setting properties from Rust to QML.
* `cargo run --example listmodel` for an example of providing QML with list model from Rust.

Requires CMake, Make, Qt (Core, Gui, Widgets, Quick) and, of course, Rust.

## In-app examples

* [Architect](https://github.com/White-Oak/architect/tree/qml-lib) - an app showing some git stats,
using qml-rust to provide properties and lists to QML in [here](https://github.com/White-Oak/architect/blob/qml-lib/src/view/qt.rs)
* [Kefia](https://github.com/White-Oak/kefia) - A simple package manager, that provides a QListModel to QML,
[here](https://github.com/White-Oak/kefia/blob/master/src/view.rs)

# Status
Done:
* Basic initialization and execution
* Providing properties for QML files
* QAbstractListModels - to provide changable models for QML items (early draft, still lacks proper mutability)

To be done:
* QObjects: slots, signals, properties
* QML singletons
* etc
