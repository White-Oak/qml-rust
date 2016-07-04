

pub trait QMetaTypable {
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

pub enum QMetaType {
    Void = 43,
    Bool = 1,
    Int = 2,
    Double = 6,
    Long = 32,
    QString = 10,
}
