# QML-rust - bindings for [Qt Quick](http://doc.qt.io/qt-5/qtquick-index.html)
[![Build
Status](https://travis-ci.org/White-Oak/qml-rust.svg?branch=master)](https://travis-ci.org/White-Oak/qml-rust)
[![Crates.io](https://img.shields.io/crates/v/qml.svg)]()

Bindings are based on [DOtherSide](https://github.com/filcuc/DOtherSide) C bindings for QML
Library is mostly feature-compliant with other bindings based on the library, but lacks some minor features and has quite a few bugs.

## [documentation](https://white-oak.github.io/qml-rust/qml/)
# Examples
All examples are located in a folder [`examples/`](examples), under `example_name.rs` and `example_name.qml` names.

* `cargo run --example properties` for setting properties from Rust to QML.
* `cargo run --example listmodel` for an example of providing QML with list model from Rust.
* `cargo run --example listmodel_macro` for the same example, but using `Q_LISTMODEL!` and `Q_LISTMODEL_ITEM!` macro.
* `cargo run --example sigslots` for an example of how to create your own `QObject` with signals and slots, and to communicate between QML and Rust. Also shows how to use `Q_OBJECT!` macro.
* `cargo run --example qobjects` for an example of how to use `Q_OBJECT!` macro with different types.
* `cargo run --example qvarlists` for an example of how to use `qvarlist!` macro to easily form `QVariant` (used to pass data to QML) of a complex array.
* `cargo run --example threaded` for an example of multithreading.
* `cargo run --example qmlregister` for an example of how to register and use your own types from Rust in QML.
* An example in `examples/resources` (should be run manually by `cargo run`) shows how to use qrc resources.

Requires CMake, Make, Qt (Core, Gui, Widgets, Quick) and, of course, Rust.

To run tests: `RUST_TEST_THREADS=1 cargo test`
## In-app examples

* [Architect](https://github.com/White-Oak/architect) - an app showing some git stats,
using qml-rust to provide properties and lists to QML in [here](https://github.com/White-Oak/architect/blob/master/src/view/qt.rs).
* [Kefia](https://github.com/White-Oak/kefia) - A simple package manager, that provides a QListModel to QML, registers a QObject with slots and communicates between QML and Rust,
[here](https://github.com/White-Oak/kefia/blob/master/src/view.rs).

# Status
Done:
* Basic initialization and execution.
* Providing properties to QML files.
* QAbstractListModels - provides changable models for QML items (early draft, still lacks proper mutability).
* QObjects: slots, signals (limited properties support). Emitting signals and receiving slots works.
* Registering your own QML types (singletons or not) from Rust code.

To be done:
* the library is mostly done, but some stuff is lacking polish, like possible memory leaks or better macro designs.
