use libc;

use qmlengine::*;
use utils::*;
use types::*;

extern "C" {
    fn dos_qvariant_create() -> DosQVariant;
    fn dos_qvariant_create_int(value: i32) -> DosQVariant;
    fn dos_qvariant_create_bool(value: bool) -> DosQVariant;
    fn dos_qvariant_create_string(value: *const libc::c_char) -> DosQVariant;
    fn dos_qvariant_create_qobject(value: DosQObject) -> DosQVariant;
    fn dos_qvariant_create_qvariant(value: DosQVariant) -> DosQVariant;
    fn dos_qvariant_create_float(value: f32) -> DosQVariant;
    fn dos_qvariant_create_double(value: f64) -> DosQVariant;

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
    ptr: DosQVariant,
    owned: bool,
}

impl Clone for QVariant {
    fn clone(&self) -> Self {
        unsafe {
            QVariant {
                ptr: dos_qvariant_create_qvariant(self.ptr),
                owned: true,
            }
        }
    }
}

use std::ffi::CString;
impl QVariant {
    pub fn to_int(&self) -> i32 {
        unsafe { dos_qvariant_toInt(self.ptr) }
    }

    pub fn into_bool(self) -> bool {
        unsafe { dos_qvariant_toBool(self.ptr) }
    }

    pub fn into_float(self) -> f32 {
        unsafe { dos_qvariant_toFloat(self.ptr) }
    }

    pub fn into_double(self) -> f64 {
        unsafe { dos_qvariant_toDouble(self.ptr) }
    }

    pub fn into_cstring(self) -> CString {
        unsafe { CString::from_raw(dos_qvariant_toString(self.ptr)) }
    }

    pub fn set(&mut self, other: &QVariant) {
        unsafe {
            dos_qvariant_assign(self.ptr as MutDosQVariant, other.ptr);
        }
    }
}

pub fn get_private_variant(from: &QVariant) -> DosQVariant {
    from.ptr
}

impl Drop for QVariant {
    fn drop(&mut self) {
        if self.owned {
            unsafe { dos_qvariant_delete(self.ptr) }
        }
    }
}

impl From<DosQObject> for QVariant {
    fn from(i: DosQObject) -> Self {
        unsafe {
            QVariant {
                ptr: dos_qvariant_create_qobject(i),
                owned: true,
            }
        }
    }
}

impl From<MutDosQVariant> for QVariant {
    fn from(vptr: MutDosQVariant) -> Self {
        QVariant {
            ptr: vptr as *const libc::c_void,
            owned: false,
        }
    }
}

impl From<i32> for QVariant {
    fn from(i: i32) -> Self {
        unsafe {
            QVariant {
                ptr: dos_qvariant_create_int(i),
                owned: true,
            }
        }
    }
}

impl From<f32> for QVariant {
    fn from(i: f32) -> Self {
        unsafe {
            QVariant {
                ptr: dos_qvariant_create_float(i),
                owned: true,
            }
        }
    }
}

impl From<f64> for QVariant {
    fn from(i: f64) -> Self {
        unsafe {
            QVariant {
                ptr: dos_qvariant_create_double(i),
                owned: true,
            }
        }
    }
}

impl From<bool> for QVariant {
    fn from(i: bool) -> Self {
        unsafe {
            QVariant {
                ptr: dos_qvariant_create_bool(i),
                owned: true,
            }
        }
    }
}

impl<'a> From<&'a str> for QVariant {
    fn from(i: &'a str) -> Self {
        unsafe {
            QVariant {
                ptr: dos_qvariant_create_string(stoptr(i)),
                owned: true,
            }
        }
    }
}

// reverse Froms
impl From<QVariant> for i32 {
    fn from(_: QVariant) -> Self {
        unimplemented!()
    }
}
