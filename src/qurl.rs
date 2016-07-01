use libc;

use utils::*;
use types::*;

extern "C" {

    // QUrl
    fn dos_qurl_create(url: *const libc::c_char, parsingMode: i32) -> DosQUrl;
// DOS_API void  DOS_CALL dos_qurl_delete(DosQUrl *vptr);
// DOS_API char *DOS_CALL dos_qurl_to_string(const DosQUrl *vptr);
// DOS_API bool dos_qurl_isValid(const DosQUrl *vptr);
}

pub fn construct_qurl(url: &str) -> DosQUrl {
    unsafe { dos_qurl_create(stoptr(url), 0) }
}
