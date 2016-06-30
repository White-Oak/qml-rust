use libc;

use std::ffi::CString;
pub fn stoptr(s: &str) -> *const libc::c_char {
    let cstr = CString::new(s).unwrap();
    cstr.into_raw() as *const i8
}
