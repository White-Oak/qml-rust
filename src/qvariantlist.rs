use qvariant::*;

pub struct QVariantList {
    vec: Vec<QVariant>,
}

impl Default for QVariantList {
    fn default() -> Self {
        QVariantList { vec: vec![] }
    }
}

impl From<QVariantList> for QVariant {
    fn from(i: QVariantList) -> Self {
        i.vec.into()
    }
}

impl From<Vec<QVariant>> for QVariantList {
    fn from(i: Vec<QVariant>) -> Self {
        QVariantList { vec: i }
    }
}

impl From<QVariantList> for Vec<QVariant> {
    fn from(i: QVariantList) -> Self {
        i.vec
    }
}

impl From<QVariant> for QVariantList {
    fn from(i: QVariant) -> Self {
        <Vec<QVariant>>::from(i).into()
    }
}
