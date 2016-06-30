use libc;

use types::*;

extern "C" {

    // DOS_API DosQModelIndex *DOS_CALL dos_qmodelindex_create();
    // DOS_API DosQModelIndex *DOS_CALL dos_qmodelindex_create_qmodelindex(DosQModelIndex *index);
    // DOS_API void DOS_CALL dos_qmodelindex_delete (DosQModelIndex *vptr);
    fn dos_qmodelindex_row(vptr: *const libc::c_void) -> i32;
    fn dos_qmodelindex_column(vptr: *const libc::c_void) -> i32;
// DOS_API bool DOS_CALL dos_qmodelindex_isValid(const DosQModelIndex *vptr);
// DOS_API DosQVariant *DOS_CALL dos_qmodelindex_data (const DosQModelIndex *vptr, int role);
// DOS_API DosQModelIndex *DOS_CALL dos_qmodelindex_parent (const DosQModelIndex *vptr);
// DOS_API DosQModelIndex *DOS_CALL dos_qmodelindex_child  (const DosQModelIndex *vptr, int row, int column);
// DOS_API DosQModelIndex *DOS_CALL dos_qmodelindex_sibling(const DosQModelIndex *vptr, int row, int column);
// DOS_API void DOS_CALL dos_qmodelindex_assign (DosQModelIndex *l, const DosQModelIndex *r);
}

pub struct QModelIndex(DosQModelIndex);

impl QModelIndex {
    pub fn row(&self) -> i32 {
        unsafe { dos_qmodelindex_row(self.0) }
    }

    pub fn column(&self) -> i32 {
        unsafe { dos_qmodelindex_column(self.0) }
    }
}

impl From<DosQModelIndex> for QModelIndex {
    fn from(i: DosQModelIndex) -> Self {
        QModelIndex(i)
    }
}
