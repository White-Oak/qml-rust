//! Contains definitions required to communicate with Qt with the use of `named types`.
//!
//! Provides definition of [`QMetaType`](enum.QMetaType.html) enum, that contains mapping of named types
//! and a trait [`QMetaTypable`](trait.QMetaTypable.html), that controls which types are able to be used in signals, slots or properties.

/// Provides an associated variant of enum for a type.
///
/// Only types that implement this, may be used as types in signals, slots or properties.
pub trait QMetaTypable {
    /// Returns an associate variant of QMetaType
    fn metatype() -> QMetaType;
}

impl QMetaTypable for i32 {
    fn metatype() -> QMetaType {
        QMetaType::Int
    }
}

impl QMetaTypable for String {
    fn metatype() -> QMetaType {
        QMetaType::QString
    }
}

impl QMetaTypable for f64 {
    fn metatype() -> QMetaType {
        QMetaType::Double
    }
}

impl QMetaTypable for f32 {
    fn metatype() -> QMetaType {
        QMetaType::Float
    }
}
/// Analogue of [`Qt::QMetaType::Type`](http://doc.qt.io/qt-5/qmetatype.html#Type-enum)
///
/// `QMetaType` in Qt manages named types in the meta-object system.
pub enum QMetaType {
    Void = 43,
    Bool = 1,
    Int = 2,
    Double = 6,
    Long = 32,
    QString = 10,
    Float = 38
}
