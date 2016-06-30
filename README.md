# QML-rust - bindings for [Qt Quick](http://doc.qt.io/qt-5/qtquick-index.html)
Library is still in a rough shape.  
Bindings are based on [DOtherSide](https://github.com/filcuc/DOtherSide) C bindings for QML

# Example
`cargo run --example properties`  
Requires CMake, Make, Qt (Core, Gui, Widgets, Quick) and, of course, Rust.

# Status
Done:
* Basic initialization and execution
* Providing properties for QML files
To be done:
* QObjects: slots, signals, properties
* QAbstractItemModels - to provide changable models for QML items
* QML singletons
* etc
