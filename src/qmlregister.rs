use libc;

use types::*;

extern "C" {
    fn dos_qdeclarative_qmlregistertype(qmlRegisterType: *const QmlRegisterType) -> i32;
    fn dos_qdeclarative_qmlregistersingletontype(qmlRegisterType: *const QmlRegisterType) -> i32;
}

#[repr(C)]
pub struct QmlRegisterType {
    major: i32,
    minor: i32,
    uri: DosCStr,
    qml: DosCStr,
    static_meta_object: DosQMetaObject,
    create_dobject: CreateDObject,
    delete_dobject: DeleteDObject,
}

pub type CreateDObject = extern "C" fn(i32, *mut DosQObject, *mut *const libc::c_void);
pub type DeleteDObject = extern "C" fn(i32, *const libc::c_void);
