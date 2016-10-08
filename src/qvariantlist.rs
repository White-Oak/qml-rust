use qvariant::*;

/// A wrapper around `Vec<QVariant>`, that is supposed to be used in [`Q_OBJECT!`](macro.Q_OBJECT.html) properties.
/// It is only needed because of macro restrictions in current stable Rust.
/// # Examples
/// ```
/// # #[macro_use] extern crate qml;
/// # use qml::*;
/// # #[allow(unused_variables)]
/// # fn main() {
/// let shortcut: QVariantList = qvarlist![["John", [2, 2]], ["Ivan", [10, 0]], ["Mary", [0, 1]]].into();
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate qml;
/// # use qml::*;
/// pub struct Example;
///
/// Q_OBJECT!(
/// pub Example as QExample{
///     signals:
///     slots:
///     properties:
///         list: QVariantList; read: get_list, write: set_list, notify: list_changed;
/// });
///
/// # fn main() {
///     let mut qobj = QExample::new(Example, qvarlist![["John", [2, 2]]].into());
/// # }
/// ```
/// #See
/// [`qvarlist!`](macro.qvarlist.html), [`Q_OBJECT!`](macro.Q_OBJECT.html)

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
