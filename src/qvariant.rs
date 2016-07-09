use libc;
use std::ffi::{CString, CStr};
use std::sync::atomic::{AtomicPtr, Ordering};

use utils::*;
use types::*;
use qobject::*;

extern "C" {
    fn dos_qvariant_create() -> DosQVariant;
    fn dos_qvariant_create_int(value: i32) -> DosQVariant;
    fn dos_qvariant_create_bool(value: bool) -> DosQVariant;
    fn dos_qvariant_create_string(value: c_str) -> DosQVariant;
    fn dos_qvariant_create_qobject(value: DosQObject) -> DosQVariant;
    fn dos_qvariant_create_qvariant(value: DosQVariant) -> DosQVariant;
    fn dos_qvariant_create_float(value: f32) -> DosQVariant;
    fn dos_qvariant_create_double(value: f64) -> DosQVariant;
    fn dos_qvariant_create_array(size: i32, array: *const DosQVariant) -> DosQVariant;

    fn dos_qvariant_toInt(val: DosQVariant) -> i32;
    fn dos_qvariant_toBool(val: DosQVariant) -> bool;
    fn dos_qvariant_toString(val: DosQVariant) -> *mut libc::c_char;
    fn dos_qvariant_toFloat(val: DosQVariant) -> f32;
    fn dos_qvariant_toDouble(val: DosQVariant) -> f64;
    // DOS_API DosQObject *DOS_CALL dos_qvariant_toQObject(const DosQVariant *vptr);

    fn dos_qvariant_isnull(val: DosQVariant) -> bool;
    fn dos_qvariant_assign(val: MutDosQVariant, other: DosQVariant);
    fn dos_qvariant_delete(val: DosQVariant);

// DOS_API void   DOS_CALL dos_qvariant_setInt    (DosQVariant *vptr, int value);
// DOS_API void   DOS_CALL dos_qvariant_setBool   (DosQVariant *vptr, bool value);
// DOS_API void   DOS_CALL dos_qvariant_setFloat  (DosQVariant *vptr, float value);
// DOS_API void   DOS_CALL dos_qvariant_setDouble (DosQVariant *vptr, double value);
// DOS_API void   DOS_CALL dos_qvariant_setString (DosQVariant *vptr, const char *value);
// DOS_API void   DOS_CALL dos_qvariant_setQObject(DosQVariant *vptr, DosQObject *value);

}

/// This holds a value to be providen for a QML context.
///
/// A value can be different: int, string, float, double, bool or even a custom object.
pub struct QVariant {
    ptr: AtomicPtr<WQVariant>,
    owned: bool,
}

impl Clone for QVariant {
    fn clone(&self) -> Self {
        unsafe {
            new_qvar(dos_qvariant_create_qvariant(self.ptr.load(Ordering::Relaxed)),
                     true)
        }
    }
}

impl QVariant {
    pub fn to_int(&self) -> i32 {
        unsafe { dos_qvariant_toInt(self.ptr.load(Ordering::Relaxed)) }
    }

    pub fn into_bool(self) -> bool {
        unsafe { dos_qvariant_toBool(self.ptr.load(Ordering::Relaxed)) }
    }

    pub fn into_float(self) -> f32 {
        unsafe { dos_qvariant_toFloat(self.ptr.load(Ordering::Relaxed)) }
    }

    pub fn into_double(self) -> f64 {
        unsafe { dos_qvariant_toDouble(self.ptr.load(Ordering::Relaxed)) }
    }

    pub fn into_cstring(self) -> CString {
        unsafe { CString::from_raw(dos_qvariant_toString(self.ptr.load(Ordering::Relaxed))) }
    }

    /// Sets the value for this `QVariant`
    pub fn set(&mut self, other: &QVariant) {
        unsafe {
            dos_qvariant_assign(self.ptr.load(Ordering::Relaxed),
                                other.ptr.load(Ordering::Relaxed));
        }
    }
}

pub fn throw(qvar: &mut QVariant, flag: bool) {
    qvar.owned = flag;
}
fn new_qvar(ptr: DosQVariant, owned: bool) -> QVariant {
    QVariant {
        ptr: AtomicPtr::new(ptr as MutDosQVariant),
        owned: owned,
    }
}

pub fn new_qvariant(ptr: DosQVariant) -> QVariant {
    new_qvar(ptr, false)
}

pub fn get_private_variant(from: &QVariant) -> DosQVariant {
    from.ptr.load(Ordering::Relaxed)
}

impl Drop for QVariant {
    fn drop(&mut self) {
        if self.owned {
            unsafe { dos_qvariant_delete(self.ptr.load(Ordering::Relaxed)) }
        }
    }
}

#[doc(hidden)]
impl From<DosQObject> for QVariant {
    fn from(i: DosQObject) -> Self {
        unsafe { new_qvar(dos_qvariant_create_qobject(i), true) }
    }
}

#[doc(hidden)]
impl From<DosQAbstractListModel> for QVariant {
    fn from(i: DosQAbstractListModel) -> Self {
        unsafe { new_qvar(dos_qvariant_create_qobject(i as DosQObject), true) }
    }
}

#[doc(hidden)]
impl From<DosQVariant> for QVariant {
    fn from(vptr: DosQVariant) -> Self {
        new_qvar(vptr, false)
    }
}

#[doc(hidden)]
impl From<MutDosQVariant> for QVariant {
    fn from(vptr: MutDosQVariant) -> Self {
        new_qvar(vptr, false)
    }
}

#[doc(hidden)]
impl From<QObject> for QVariant {
    fn from(i: QObject) -> Self {
        unsafe { new_qvar(dos_qvariant_create_qobject(get_qobj_ptr(&i)), true) }
    }
}

impl<'a> From<&'a [QVariant]> for QVariant {
    fn from(i: &'a [QVariant]) -> Self {
        unsafe {
            let vec = i.iter()
                .map(|qvar| qvar.ptr.load(Ordering::Relaxed) as DosQVariant)
                .collect::<Vec<DosQVariant>>();
            let ptr = vec.as_ptr();
            forget(vec);
            new_qvar(dos_qvariant_create_array(i.len() as i32, ptr), true)
        }
    }
}

use std::mem::forget;

impl From<Vec<QVariant>> for QVariant {
    fn from(i: Vec<QVariant>) -> Self {
        unsafe {
            let len = i.len();
            let vec = i.iter()
                .map(|qvar| qvar.ptr.load(Ordering::Relaxed) as DosQVariant)
                .collect::<Vec<DosQVariant>>();
            let ptr = vec.as_ptr();
            forget(vec);
            new_qvar(dos_qvariant_create_array(len as i32, ptr), true)
        }
    }
}

#[doc(hidden)]
impl<'a> From<&'a QObject> for QVariant {
    fn from(i: &'a QObject) -> Self {
        unsafe { new_qvar(dos_qvariant_create_qobject(get_qobj_ptr(i)), true) }
    }
}

impl From<i32> for QVariant {
    fn from(i: i32) -> Self {
        unsafe { new_qvar(dos_qvariant_create_int(i), true) }
    }
}

impl From<f32> for QVariant {
    fn from(i: f32) -> Self {
        unsafe { new_qvar(dos_qvariant_create_float(i), true) }
    }
}

impl From<f64> for QVariant {
    fn from(i: f64) -> Self {
        unsafe { new_qvar(dos_qvariant_create_double(i), true) }
    }
}

impl From<bool> for QVariant {
    fn from(i: bool) -> Self {
        unsafe { new_qvar(dos_qvariant_create_bool(i), true) }
    }
}

impl<'a> From<&'a str> for QVariant {
    fn from(i: &'a str) -> Self {
        unsafe {
            let ptr = stoptr(i);
            let qvar = new_qvar(dos_qvariant_create_string(ptr), true);
            // Dropping CString
            ptrtos(ptr);
            qvar
        }
    }
}

impl From<String> for QVariant {
    fn from(i: String) -> Self {
        QVariant::from(i.as_str())
    }
}

// reverse Froms
impl From<QVariant> for i32 {
    fn from(i: QVariant) -> Self {
        unsafe { dos_qvariant_toInt(i.ptr.load(Ordering::Relaxed)) }
    }
}

impl From<QVariant> for String {
    fn from(i: QVariant) -> Self {
        // Should i get ownership or not?
        unsafe {
            CStr::from_ptr(dos_qvariant_toString(i.ptr.load(Ordering::Relaxed)))
                .to_string_lossy()
                .into_owned()
        }
    }
}
