use libc;
use std::collections::HashMap;

use qvariant::*;
use types::*;
use qurl::*;
use qobject::*;
use qmeta::*;

extern "C" {
    fn dos_qapplication_create();
    fn dos_qapplication_exec();
    fn dos_qapplication_quit();
    fn dos_qapplication_delete();

    fn dos_qqmlapplicationengine_create() -> QQmlApplicationEngine;
    fn dos_qqmlapplicationengine_load(vptr: QQmlApplicationEngine, filename: *const libc::c_char);
    fn dos_qqmlapplicationengine_load_url(vptr: QQmlApplicationEngine, url: DosQUrl);
    fn dos_qqmlapplicationengine_load_data(vptr: QQmlApplicationEngine,
                                           data: *const libc::c_char);
    // fn dos_qqmlapplicationengine_add_import_path(vptr: *mut DosQQmlApplicationEngine, const char *path);
    fn dos_qqmlapplicationengine_context(vptr: QQmlApplicationEngine) -> DosQQmlContext;
    fn dos_qqmlapplicationengine_delete(vptr: QQmlApplicationEngine);

    fn dos_qqmlcontext_setcontextproperty(vptr: DosQQmlContext,
                                          name: *const libc::c_char,
                                          value: DosQVariant);

}

/// Provides an entry point for building QML applications from Rust
pub struct QmlEngine {
    ptr: QQmlApplicationEngine,
    stored: Vec<QVariant>,
    objs: HashMap<String, Vec<Box<QObject>>>,
}

impl QmlEngine {
    /// Creates a QML context of a non-headless application
    pub fn new() -> Self {
        unsafe {
            dos_qapplication_create();
            QmlEngine {
                ptr: dos_qqmlapplicationengine_create(),
                stored: Vec::new(),
                objs: HashMap::new(),
            }
        }
    }

    /// Loads a file as a qml file
    pub fn load_file(&self, path: &str) {
        let path_raw = ::std::env::current_dir().unwrap().join(path);
        let path = if cfg!(windows) {
            format!("file:///{}", path_raw.display())
        } else {
            format!("file://{}", path_raw.display())
        };
        unsafe { dos_qqmlapplicationengine_load_url(self.ptr, construct_qurl(&path)) }
    }

    /// Loads a string as a qml file
    pub fn load_data(&self, data: &str) {
        unsafe { dos_qqmlapplicationengine_load_data(self.ptr, stoptr(data)) }
    }

    /// Launches the application
    pub fn exec(&self) {
        unsafe {
            dos_qapplication_exec();
        }
    }
    /// Closes the application
    pub fn quit(&self) {
        unsafe {
            dos_qapplication_quit();
        }
    }

    /// Sets a property for this QML context
    ///
    /// This variant stores qvariant, so it is removed, only when this QmlEngine is removed.
    pub fn set_and_store_property<T: Into<QVariant>>(&mut self, name: &str, value: T) {
        let val = value.into();
        unsafe {
            let context = dos_qqmlapplicationengine_context(self.ptr);
            dos_qqmlcontext_setcontextproperty(context, stoptr(name), get_private_variant(&val));
        }
        self.stored.push(val);
    }

    /// Sets a property for this QML context
    pub fn set_property(&self, name: &str, value: &QVariant) {
        unsafe {
            let context = dos_qqmlapplicationengine_context(self.ptr);
            dos_qqmlcontext_setcontextproperty(context, stoptr(name), get_private_variant(value));
        }
    }
}

pub fn find_qobject<'a, T>(qqae: &'a QmlEngine, name: &str, obj: &T) -> &'a Box<QObject>
    where T: QObjectMacro
{
    if let Some(v) = qqae.objs.get(name) {
        let iter = v.iter();
        for i in iter {
            if i.obj == (obj as *const T as *const libc::c_void) {
                return i;
            }
        }
        unreachable!()
    } else {
        panic!("There is no '{}' stored in this QmlEngine!", name);
    }
}

pub fn add_qobject<'a>(qqae: &'a mut QmlEngine,
                       name: &str,
                       qobj: Box<QObject>)
                       -> &'a mut Box<QObject> {
    if qqae.objs.contains_key(name) {
        let mut v = qqae.objs.get_mut(name).unwrap();
        v.push(qobj);
        v.last_mut().unwrap()
    } else {
        qqae.objs.insert(name.clone().into(), vec![qobj]);
        qqae.objs.get_mut(name).unwrap().last_mut().unwrap()
    }
}
use utils::*;

impl Default for QmlEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for QmlEngine {
    fn drop(&mut self) {
        unsafe {
            dos_qapplication_quit();
            dos_qqmlapplicationengine_delete(self.ptr);
            dos_qapplication_delete();
        }
    }
}
