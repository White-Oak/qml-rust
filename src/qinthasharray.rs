use libc;

use utils::*;
use types::*;

extern "C" {

    fn dos_qhash_int_qbytearray_create() -> MutDosQHashIntQByteArray;
    fn dos_qhash_int_qbytearray_delete(vptr: MutDosQHashIntQByteArray);
    fn dos_qhash_int_qbytearray_insert(vptr: MutDosQHashIntQByteArray,
                                       key: i32,
                                       value: *const libc::c_char);
    fn dos_qhash_int_qbytearray_value(vptr: MutDosQHashIntQByteArray,
                                      key: i32)
                                      -> *const libc::c_char;
}

pub struct QHashIntQByteArray(MutDosQHashIntQByteArray);

impl QHashIntQByteArray {
    pub fn new() -> Self {
        unsafe { QHashIntQByteArray(dos_qhash_int_qbytearray_create()) }
    }

    pub fn insert(&self, key: i32, value: &str) {
        unsafe { dos_qhash_int_qbytearray_insert(self.0, key, stoptr(value)) }
    }
}

// impl Drop for QHashIntQByteArray {
//     fn drop(&mut self) {
//         unsafe { dos_qhash_int_qbytearray_delete(self.0) }
//     }
// }

pub fn get_dqhiqba_ptr(o: QHashIntQByteArray) -> MutDosQHashIntQByteArray {
    o.0
}

impl From<MutDosQHashIntQByteArray> for QHashIntQByteArray {
    fn from(i: MutDosQHashIntQByteArray) -> Self {
        QHashIntQByteArray(i)
    }
}
