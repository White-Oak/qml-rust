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

pub type CreateDObject = extern "C" fn(i32, DosQObject, *mut *const libc::c_void, *mut DosQObject);
pub type DeleteDObject = extern "C" fn(i32, *const libc::c_void);


extern "C" fn delete_dobject(id: i32, ptr: *const libc::c_void) {}

/// Provides definitions for a type that can be used from QML.
///
/// The same macro is used to prepare a type for being used as a normal type or a singleton.
/// The only requirement is that the type in question should provide `Default` implementation.
/// # Examples
/// ```
///
/// #[derive(Default)]
/// pub struct Test;
///
/// Q_OBJECT!(
/// pub Test as QTest{
///     signals:
///     slots:
///     properties:
///         name: String; read: get_name, write: set_name, notify: name_changed;
/// });
///
/// Q_REGISTERABLE_QML!(QTest: Test as TestRsObject 1=>0, from TestModule);
/// ```
/// Later on a type that was made registerable can be used in [`Q_REGISTER_QML`](macro.Q_REGISTER_QML!.html)
/// or in [`Q_REGISTER_SINGLETON_QML`](macro.Q_REGISTER_SINGLETON_QML!.html) macros to be used as a type in QML.
#[macro_export]
macro_rules! Q_REGISTERABLE_QML(
    ($wrapper:ident : $origin:ident as $qml:ident $major:expr=>$minor:expr, from $uri:ident) => {
        impl QMLRegisterable for $wrapper{
            fn qualify_to_register(&self) ->  (i32, i32, &'static str, &'static str) {
                ($major, $minor, stringify!($uri), stringify!($qml))
            }

            fn get_new(&self) -> *mut c_void {
                unsafe {
                    let obj = $wrapper::with_no_props($origin::default());
                    let res = Box::into_raw(obj) as *mut c_void;
                    res
                }
            }

            fn get_qobj_from_ptr(&self, ptr: *mut c_void) -> *mut QObject {
                unsafe {
                    let mut obj: Box<$wrapper> = Box::from_raw(ptr as *mut $wrapper);
                    let res = obj.get_qobj_mut() as *mut QObject;
                    ::std::mem::forget(obj);
                    res
                }
            }
        }

        impl $wrapper {
            pub fn get_shallow() -> Self {
                unsafe {
                    ::std::mem::uninitialized()
                }
            }
        }
    }
);

/// Registers a type as a QML type.
///
/// To use this macro [`Q_REGISTERABLE_QML`](macro.Q_REGISTERABLE_QML!.html) should be used first.
/// # Examples
/// ```
///
/// #[derive(Default)]
/// pub struct Test;
///
/// Q_OBJECT!(
/// pub Test as QTest{
///     signals:
///     slots:
///     properties:
///         name: String; read: get_name, write: set_name, notify: name_changed;
/// });
///
/// Q_REGISTERABLE_QML!(QTest: Test as TestRsObject 1=>0, from TestModule);
///
/// // ...
///
/// # fn main() {
/// Q_REGISTER_QML!(QTest);
/// # }
/// ```
/// Then in qml:
///
/// ```qml
/// import TestModule 1.0
///
/// TestRsObject{
///     name: "Oak"
/// }
/// ```
#[macro_export]
macro_rules! Q_REGISTER_QML(
        ($wrapper:ident) => {
            register_qml_type($wrapper::get_shallow());
        }
);

/// Registers a type as a singleton type in QML.
///
/// To use this macro [`Q_REGISTERABLE_QML`](macro.Q_REGISTERABLE_QML!.html) should be used first.
/// # Examples
/// ```
///
/// #[derive(Default)]
/// pub struct Test;
///
/// Q_OBJECT!(
/// pub Test as QTest{
///     signals:
///     slots:
///     properties:
///         name: String; read: get_name, write: set_name, notify: name_changed;
/// });
///
/// Q_REGISTERABLE_QML!(QTest: Test as TestRsSingleton 1=>0, from TestModule);
///
/// // ...
///
/// # fn main() {
/// Q_REGISTER_SINGLETON_QML!(QTest);
/// # }
/// ```
/// Then in qml:
///
/// ```qml
/// import TestModule 1.0
///
/// Item {
///     Component.onCompleted: {
///         console.log(TestRsSingleton.name)
///     }
/// }
/// ```
#[macro_export]
macro_rules! Q_REGISTER_SINGLETON_QML(
        ($wrapper:ident) => {
            register_qml_singleton_type($wrapper::get_shallow());
        }
);

pub type RegisterQualifier = (i32, i32, &'static str, &'static str);
#[doc(hidden)]
pub trait QMLRegisterable: QObjectMacro {
    fn qualify_to_register(&self) -> RegisterQualifier;
    fn get_new(&self) -> *mut libc::c_void;
    fn get_qobj_from_ptr(&self, ptr: *mut libc::c_void) -> *mut QObject;
}

use qobject::*;
use qmeta::*;
use utils::*;
use std::mem::{forget, transmute};
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::collections::HashMap;

extern "C" fn create_dobject(id: i32,
                             wrapper: DosQObject,
                             binded_ptr: *mut *const libc::c_void,
                             dosQObject: *mut DosQObject) {
    let sing = singleton();
    let map = sing.inner.lock().unwrap();
    // Getting shallow object from the map
    let shallow = map.get(&id).unwrap();
    // Getting pointer to a created object
    let binded = shallow.get_new();

    // Returning pointers to a wrapper and to an DosQObject, then swapping DosQObject with a fresh one
    // Comments are copied 'as is' from the DOtherSide docs to ensure correctness
    unsafe {
        let mut qobj = &mut *shallow.get_qobj_from_ptr(binded);
        // # Retrieve the DosQObject created dos_qobject_create() inside the nimQObject
        *dosQObject = get_qobj_ptr(qobj);
        // # Store the pointer to the nimQObject
        *binded_ptr = get_binded_ptr(qobj);
        // # Swap the vptr inside the nimQObject with the wrapper
        set_qobj_ptr(qobj, wrapper);
    }
    forget(binded);
}


#[derive(Clone)]
struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    inner: Arc<Mutex<HashMap<i32, Box<QMLRegisterable>>>>,
}

fn singleton() -> SingletonReader {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = SingletonReader { inner: Arc::new(Mutex::new(HashMap::new())) };

            // Put it in the heap so it can outlive this call
            SINGLETON = transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

type Registerer = unsafe extern "C" fn(*const QmlRegisterType) -> i32;
fn register_with<T: QMLRegisterable + 'static>(t: T, r: Registerer) {
    let (major, minor, uri, qml) = t.qualify_to_register();
    let qmeta = QMetaDefinition::new(t.qmeta());
    let meta = QMeta::new_for_qobject(qmeta);
    let sing = singleton();
    let mut map = sing.inner.lock().unwrap();

    let qrt = QmlRegisterType {
        major: major,
        minor: minor,
        uri: stoptr(uri),
        qml: stoptr(qml),
        static_meta_object: get_dos_qmeta(&meta),
        create_dobject: create_dobject,
        delete_dobject: delete_dobject,
    };
    forget(meta);
    let id = unsafe { r(&qrt as *const QmlRegisterType) };
    map.insert(id, Box::new(t));
    forget(qrt);
}

pub fn register_qml_type<T: QMLRegisterable + 'static>(t: T) {
    register_with(t, dos_qdeclarative_qmlregistertype)
}

pub fn register_qml_singleton_type<T: QMLRegisterable + 'static>(t: T) {
    register_with(t, dos_qdeclarative_qmlregistersingletontype)
}
