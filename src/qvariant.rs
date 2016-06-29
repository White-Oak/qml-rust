use libc;

type DosQVariant = *const libc::c_void;

extern "C" {
    fn dos_qvariant_create() -> DosQVariant;
    fn dos_qvariant_create_int(value: i32) -> DosQVariant;
    fn dos_qvariant_create_bool(value: bool) -> DosQVariant;
    fn dos_qvariant_create_string(value: *const libc::c_char) -> DosQVariant;
    // fn DosQVariant  dos_qvariant_create_qobject(DosQObject *value)->DosQVariant;
    // fn DosQVariant  dos_qvariant_create_qvariant(const DosQVariant *value)->DosQVariant;
    fn dos_qvariant_create_float(value: f32) -> DosQVariant;
    fn dos_qvariant_create_double(value: f64) -> DosQVariant;

    fn dos_qvariant_toInt(val: DosQVariant) -> i32;
    fn dos_qvariant_toBool(val: DosQVariant) -> bool;
    fn dos_qvariant_toString(val: DosQVariant) -> *mut libc::c_char;
    fn dos_qvariant_toFloat(val: DosQVariant) -> f32;
    fn dos_qvariant_toDouble(val: DosQVariant) -> f64;
}

use std::marker::PhantomData;
pub struct QVariant<T>(DosQVariant, PhantomData<T>);

impl From<i32> for QVariant<i32> {
    fn from(i: i32) -> Self {
        unsafe { QVariant(dos_qvariant_create_int(i), PhantomData) }
    }
}

impl From<f32> for QVariant<f32> {
    fn from(i: f32) -> Self {
        unsafe { QVariant(dos_qvariant_create_float(i), PhantomData) }
    }
}

impl From<f64> for QVariant<f64> {
    fn from(i: f64) -> Self {
        unsafe { QVariant(dos_qvariant_create_double(i), PhantomData) }
    }
}

impl From<bool> for QVariant<bool> {
    fn from(i: bool) -> Self {
        unsafe { QVariant(dos_qvariant_create_bool(i), PhantomData) }
    }
}

impl<'a> From<&'a str> for QVariant<&'a str> {
    fn from(i: &'a str) -> Self {
        unsafe {
            QVariant(dos_qvariant_create_string(i.as_ptr() as *const i8),
                     PhantomData)
        }
    }
}

impl Into<i32> for QVariant<i32> {
    fn into(self) -> i32 {
        unsafe { dos_qvariant_toInt(self.0) }
    }
}

impl Into<bool> for QVariant<bool> {
    fn into(self) -> bool {
        unsafe { dos_qvariant_toBool(self.0) }
    }
}

impl Into<f32> for QVariant<f32> {
    fn into(self) -> f32 {
        unsafe { dos_qvariant_toFloat(self.0) }
    }
}

impl Into<f64> for QVariant<f64> {
    fn into(self) -> f64 {
        unsafe { dos_qvariant_toDouble(self.0) }
    }
}

use std::ffi::CString;
impl<'a> Into<CString> for QVariant<&'a str> {
    fn into(self) -> CString {
        unsafe { CString::from_raw(dos_qvariant_toString(self.0)) }
    }
}
