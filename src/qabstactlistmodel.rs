use libc;

use qvariant::*;
pub type DosQMetaObject = *const libc::c_void;
pub type DosQAbstractListModel = *const libc::c_void;
pub type DosQModelIndex = *const libc::c_void;
pub type MutDosQVariant = *mut libc::c_void;
type RustQALM = *const QAbstractListModel;
type DosQHashIntQByteArray = *const libc::c_void;
// type DObjectCallback = Fn (SELF???, slotname: DosQVariant, argc: i32, argv: *const DosQVariant);
extern "C" {

    fn dos_qabstractlistmodel_qmetaobject() -> DosQMetaObject;
    fn dos_qabstractlistmodel_create(callbackObject: RustQALM,
                                     metaObject: DosQMetaObject,
                                     dObjectCallback: DObjectCallback,
                                     rowCountCallback: RowCountCallback,
                                     columnCountCallback: ColumnCountCallback,
                                     dataCallback: DataCallback,
                                     setDataCallback: SetDataCallback,
                                     roleNamesCallback: RoleNamesCallback,
                                     flagsCallback: FlagsCallback,
                                     headerDataCallback: HeaderDataCallback)
                                     -> DosQAbstractListModel;
}

use std::ptr::null_mut;
pub fn RUN_QALM() -> QAbstractListModel {
    unsafe {
        let mut qalm = QAbstractListModel::new();
        let dqmo = dos_qabstractlistmodel_qmetaobject();
        let dqalm = dos_qabstractlistmodel_create(&qalm as RustQALM,
                                                  dqmo,
                                                  RustObjectCallback,
                                                  RustRowCountCallback,
                                                  RustColumnCountCallback,
                                                  RustDataCallback,
                                                  RustSetDataCallback,
                                                  RustRoleNamesCallback,
                                                  RustFlagsCallback,
                                                  RustHeaderDataCallback);
        qalm.0 = dqalm;
        qalm
    }
}

pub struct QAbstractListModel(pub DosQAbstractListModel);

impl QAbstractListModel {
    fn new() -> Self {
        QAbstractListModel(null_mut())
    }

    pub fn get_qvar(&self) -> QVariant {
        self.0.into()
    }
}
/// Called when a slot should be executed
/// @param self The pointer to the QObject in the binded language
/// @param slotName The slotName as DosQVariant. It should not be deleted
/// @param argc The number of arguments
/// @param argv An array of DosQVariant pointers. They should not be deleted
extern "C" fn RustObjectCallback(Qself: RustQALM,
                                 slotname: DosQVariant,
                                 argc: i32,
                                 argv: *const DosQVariant) {
    println!("SLOT WAS EXECUTED. hi");
}
type DObjectCallback = extern "C" fn(RustQALM, DosQVariant, i32, *const DosQVariant);

/// Called when the QAbstractListModel::rowCount method must be executed
/// @param self The pointer to the QAbstractListModel in the binded language
/// @param index The parent DosQModelIndex. It should not be deleted
/// @param result The rowCount result. This must be deferenced and filled from the binded language.
/// It should not be deleted
extern "C" fn RustRowCountCallback(Qself: RustQALM, parent: DosQModelIndex, result: *mut i32) {
    println!("ROW COUNT GOT");
    unsafe {
        *result = 0;
    }
}
type RowCountCallback = extern "C" fn(RustQALM, DosQModelIndex, *mut i32);

/// Called when the QAbstractListModel::columnCount method must be executed
/// @param self The pointer to the QAbstractListModel in the binded language
/// @param index The parent DosQModelIndex. It should not be deleted
/// @param result The rowCount result. This must be deferenced and filled from the binded language.
/// It should not be deleted
extern "C" fn RustColumnCountCallback(Qself: RustQALM, parent: DosQModelIndex, result: *mut i32) {
    println!("COLUMN COUNT GOT");
    unsafe {
        *result = 0;
    }
}
type ColumnCountCallback = extern "C" fn(RustQALM, DosQModelIndex, *mut i32);

/// Called when the QAbstractListModel::data method must be executed
/// @param self The pointer to the QAbstractListModel in the binded language
/// @param index The DosQModelIndex to which we request the data. It should not be deleted
/// @param result The DosQVariant result. This must be deferenced and filled from the binded language.
/// It should not be deleted. See dos_qvariant_assign or other DosQVariant setters
extern "C" fn RustDataCallback(Qself: RustQALM,
                               parent: DosQModelIndex,
                               role: i32,
                               result: MutDosQVariant) {
    println!("DATA CALLBACK IS HERE");
}
type DataCallback = extern "C" fn(RustQALM, DosQModelIndex, i32, MutDosQVariant);

extern "C" fn RustSetDataCallback(Qself: RustQALM,
                                  index: DosQModelIndex,
                                  value: DosQVariant,
                                  role: i32,
                                  result: *mut bool) {
    println!("SET DATA HELLO");
}
type SetDataCallback = extern "C" fn(RustQALM, DosQModelIndex, DosQVariant, i32, *mut bool);

extern "C" fn RustRoleNamesCallback(Qself: RustQALM, result: DosQHashIntQByteArray) {
    println!("HOHO ROLENAMES");
}
type RoleNamesCallback = extern "C" fn(RustQALM, DosQHashIntQByteArray);

extern "C" fn RustFlagsCallback(Qself: RustQALM, index: DosQModelIndex, result: *mut i32) {
    println!("IVE GOT FLAGS CALLBACK");
}
type FlagsCallback = extern "C" fn(RustQALM, DosQModelIndex, *mut i32);

extern "C" fn RustHeaderDataCallback(Qself: RustQALM,
                                     section: i32,
                                     orientation: i32,
                                     role: i32,
                                     result: MutDosQVariant) {
    println!("FINAL CALLBACK");
}
type HeaderDataCallback = extern "C" fn(RustQALM, i32, i32, i32, MutDosQVariant);
