use libc;

pub type DosQVariant = *const WQVariant;
pub type MutDosQVariant = *mut WQVariant;
pub type DosQObject = *mut WQObject;
pub type DosQmlApplicationEngine = *mut WQmlApplicationEngine;
pub type DosQQmlContext = *const WQQmlContext;
pub type DosQModelIndex = *const WQModelIndex;
pub type MutDosQHashIntQByteArray = *mut WQHashIntQByteArray;
pub type DosQMetaObject = *const WQMetaObject;
pub type DosQAbstractListModel = *mut WQAbstractListModel;
pub type DosQUrl = *mut WQUrl;

pub type DosCStr = *const libc::c_char;

pub enum WQVariant {}
pub enum WQObject {}
pub enum WQmlApplicationEngine {}
pub enum WQQmlContext {}
pub enum WQModelIndex {}
pub enum WQHashIntQByteArray {}
pub enum WQMetaObject {}
pub enum WQAbstractListModel {}
pub enum WQUrl {}
