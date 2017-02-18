use libc;

use std::ffi::CString;
pub fn stoptr(s: &str) -> *const libc::c_char {
    let cstr = CString::new(s).unwrap();
    cstr.into_raw() as *const libc::c_char
}

pub fn ptrtos(ptr: *const libc::c_char) -> CString {
    unsafe { CString::from_raw(ptr as *mut libc::c_char) }
}
