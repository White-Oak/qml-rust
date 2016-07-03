use libc;

use qvariant::*;
use utils::*;
use types::*;


pub struct QObject {
    ptr: DosQObject,
}
