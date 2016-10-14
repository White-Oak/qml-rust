use libc;
use std::ptr::null_mut;

use qvariant::*;
use types::*;
use qmodelindex::*;
use qinthasharray::*;

extern "C" {

    fn dos_qabstractlistmodel_qmetaobject() -> DosQMetaObject;
    fn dos_qabstractlistmodel_create(callbackObject: *const libc::c_void,
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

/// Called when a slot should be executed
/// @param self The pointer to the `QObject` in the binded language
/// @param slotName The slotName as `DosQVariant`. It should not be deleted
/// @param argc The number of arguments
/// @param argv An array of `DosQVariant` pointers. They should not be deleted
extern "C" fn RustObjectCallback(Qself: *const libc::c_void,
                                 slotname: DosQVariant,
                                 argc: i32,
                                 argv: *const DosQVariant) {
    // println!("SLOT WAS EXECUTED. hi");
}
pub type DObjectCallback = extern "C" fn(*const libc::c_void, DosQVariant, i32, *const DosQVariant);

/// Called when the `QAbstractListModel::rowCount` method must be executed
/// @param self The pointer to the `QAbstractListModel` in the binded language
/// @param index The parent `DosQModelIndex`. It should not be deleted
/// @param result The rowCount result. This must be deferenced and filled from the binded language.
/// It should not be deleted
pub type RowCountCallback = extern "C" fn(*const libc::c_void, DosQModelIndex, *mut i32);

/// Called when the `QAbstractListModel::columnCount` method must be executed
/// @param self The pointer to the `QAbstractListModel` in the binded language
/// @param index The parent `DosQModelIndex`. It should not be deleted
/// @param result The rowCount result. This must be deferenced and filled from the binded language.
/// It should not be deleted
extern "C" fn RustColumnCountCallback(Qself: *const libc::c_void,
                                      parent: DosQModelIndex,
                                      result: *mut i32) {
}
pub type ColumnCountCallback = extern "C" fn(*const libc::c_void, DosQModelIndex, *mut i32);

/// Called when the `QAbstractListModel::data` method must be executed
/// @param self The pointer to the `QAbstractListModel` in the binded language
/// @param index The `DosQModelIndex` to which we request the data. It should not be deleted
/// @param result The `DosQVariant` result. This must be deferenced and filled from the binded language.
/// It should not be deleted. See `dos_qvariant_assign` or other `DosQVariant` setters
pub type DataCallback = extern "C" fn(*const libc::c_void, DosQModelIndex, i32, MutDosQVariant);

extern "C" fn RustSetDataCallback(Qself: *const libc::c_void,
                                  index: DosQModelIndex,
                                  value: DosQVariant,
                                  role: i32,
                                  result: *mut bool) {
    // println!("SET DATA HELLO");
}
pub type SetDataCallback = extern "C" fn(*const libc::c_void,
                                         DosQModelIndex,
                                         DosQVariant,
                                         i32,
                                         *mut bool);

pub type RoleNamesCallback = extern "C" fn(*const libc::c_void, MutDosQHashIntQByteArray);

extern "C" fn RustFlagsCallback(Qself: *const libc::c_void,
                                index: DosQModelIndex,
                                result: *mut i32) {
    // println!("IVE GOT FLAGS CALLBACK");
}
pub type FlagsCallback = extern "C" fn(*const libc::c_void, DosQModelIndex, *mut i32);

extern "C" fn RustHeaderDataCallback(Qself: *const libc::c_void,
                                     section: i32,
                                     orientation: i32,
                                     role: i32,
                                     result: MutDosQVariant) {
    // println!("FINAL CALLBACK");
}
pub type HeaderDataCallback = extern "C" fn(*const libc::c_void, i32, i32, i32, MutDosQVariant);

use std::sync::atomic::{AtomicPtr, Ordering};
/// Allows providing a custom model to QML
pub struct QListModel<'a> {
    wrapped: AtomicPtr<WQAbstractListModel>,
    model: Vec<Vec<QVariant>>,
    rolenames: Vec<&'a str>,
}

extern "C" {
    fn dos_qabstractlistmodel_beginInsertRows(vptr: DosQAbstractListModel,
                                              parent: DosQModelIndex,
                                              first: i32,
                                              last: i32);
   fn dos_qabstractlistmodel_beginRemoveRows(vptr: DosQAbstractListModel,
                                             parent: DosQModelIndex,
                                             first: i32,
                                             last: i32);

    fn dos_qabstractlistmodel_endInsertRows(vptr: DosQAbstractListModel);

    fn dos_qabstractlistmodel_beginResetModel(vptr: DosQAbstractListModel);
    fn dos_qabstractlistmodel_endResetModel(vptr: DosQAbstractListModel);
    fn dos_qabstractlistmodel_endRemoveRows(vptr: DosQAbstractListModel);
}

impl<'a> QListModel<'a> {
    /// Rolenames are roles of provided data, that are mapped to corresponding roles in QML.
    pub fn new<'b>(rolenames: &'b [&'a str]) -> Box<Self> {
        unsafe {
            let mut rs = Vec::new();
            rs.extend_from_slice(rolenames);
            let result = QListModel {
                wrapped: AtomicPtr::new(null_mut()),
                model: Vec::new(),
                rolenames: rs,
            };
            // Probably need an explanation on why do I need a box
            let mut boxer = Box::new(result);

            let dqmo = dos_qabstractlistmodel_qmetaobject();
            let dqalm =
                dos_qabstractlistmodel_create(&*boxer as *const QListModel as *const libc::c_void,
                                              dqmo,
                                              RustObjectCallback, // no need
                                              RustRowCountCallback,
                                              RustColumnCountCallback, // no need
                                              RustDataCallback,
                                              RustSetDataCallback, // no need
                                              RustRoleNamesCallback,
                                              RustFlagsCallback, // no need
                                              RustHeaderDataCallback);// no need
            boxer.wrapped = AtomicPtr::new(dqalm);

            boxer
        }
    }

    /// Returns an amount of rows in this model
    pub fn row_count(&self) -> usize {
        self.model.len()
    }

    /// Gets a `QVariant` associate
    pub fn get_qvar(&self) -> QVariant {
        self.wrapped.load(Ordering::Relaxed).into()
    }

    /// Inserts a row into model
    ///
    /// Note that it clones all incoming qvariants as modifying them is not allowed.
    pub fn insert_row<T>(&mut self, qvars: T)
        where T: Iterator<Item = QVariant>
    {
        unsafe {
            let index = QModelIndex::new();
            dos_qabstractlistmodel_beginInsertRows(self.wrapped.load(Ordering::Relaxed),
                                                   get_model_ptr(&index),
                                                   self.model.len() as i32,
                                                   (self.model.len() ) as i32);
            self.model.push(qvars.collect());
            dos_qabstractlistmodel_endInsertRows(self.wrapped.load(Ordering::Relaxed));
        }
    }

    /// Remove a line from the model.
    pub fn remove_row(&mut self, index: usize)
    {
        unsafe {
            let modelindex = QModelIndex::new();
            dos_qabstractlistmodel_beginRemoveRows(self.wrapped.load(Ordering::Relaxed),
                                                   get_model_ptr(&modelindex),
                                                   index as i32,
                                                   index as i32);
            self.model.remove(index as usize);
            dos_qabstractlistmodel_endRemoveRows(self.wrapped.load(Ordering::Relaxed));
        }
    }

    /// Sets a data for this QAbstractListModel
    pub fn set_data(&mut self, qvars: Vec<Vec<QVariant>>) {
        unsafe {
            dos_qabstractlistmodel_beginResetModel(self.wrapped.load(Ordering::Relaxed));
            self.model = qvars;
            dos_qabstractlistmodel_endResetModel(self.wrapped.load(Ordering::Relaxed));
        }
    }

    /// Changes a line in underlying data
    pub fn change_line(&mut self, index: usize, qvars: Vec<QVariant>) {
        unsafe {
            dos_qabstractlistmodel_beginResetModel(self.wrapped.load(Ordering::Relaxed));
            self.model[index] = qvars;
            dos_qabstractlistmodel_endResetModel(self.wrapped.load(Ordering::Relaxed));
        }
    }

    /// Clear all the data from the model
    pub fn clear(&mut self) {
        unsafe {
            dos_qabstractlistmodel_beginResetModel(self.wrapped.load(Ordering::Relaxed));
            self.model.clear();
            dos_qabstractlistmodel_endResetModel(self.wrapped.load(Ordering::Relaxed));
        }
    }

    /// Gets an immutable view of the data
    pub fn view_data(&self) -> &[Vec<QVariant>] {
        &self.model
    }


}

impl<'a, 'b> From<&'a QListModel<'b>> for QVariant {
    fn from(i: &QListModel) -> QVariant {
        i.get_qvar()
    }
}

extern "C" fn RustRowCountCallback(Qself: *const libc::c_void,
                                   index: DosQModelIndex,
                                   result: *mut i32) {
    unsafe {
        let qlist = &*(Qself as *const QListModel);
        *result = qlist.row_count() as i32;
    }
}

extern "C" fn RustDataCallback(Qself: *const libc::c_void,
                               index: DosQModelIndex,
                               role: i32,
                               result: MutDosQVariant) {
    let qindex: QModelIndex = index.into();
    unsafe {
        let qlist = &*(Qself as *const QListModel);
        let data = &qlist.model[qindex.row() as usize][(role - START_ROLE) as usize];
        let mut qvar: QVariant = result.into();
        qvar.set(data);
    }
}

const START_ROLE: i32 = 0x0100;
extern "C" fn RustRoleNamesCallback(Qself: *const libc::c_void, result: MutDosQHashIntQByteArray) {
    unsafe {
        let qlist = &*(Qself as *const QListModel);
        let hash: QHashIntQByteArray = result.into();
        for (i, name) in qlist.rolenames.iter().enumerate() {
            hash.insert(START_ROLE + i as i32, name);
        }
    }
}
