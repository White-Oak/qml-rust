
use qvariant::*;
use types::*;
use qurl::*;

extern "C" {
    fn dos_qguiapplication_create();
    fn dos_qguiapplication_exec();
    fn dos_qguiapplication_quit();
    fn dos_qguiapplication_delete();

    fn dos_qqmlapplicationengine_create() -> DosQmlApplicationEngine;
    fn dos_qqmlapplicationengine_load(vptr: DosQmlApplicationEngine, filename: DosCStr);
    fn dos_qqmlapplicationengine_load_url(vptr: DosQmlApplicationEngine, url: DosQUrl);
    fn dos_qqmlapplicationengine_load_data(vptr: DosQmlApplicationEngine, data: DosCStr);
    fn dos_qqmlapplicationengine_add_import_path(vptr: DosQmlApplicationEngine, path: DosCStr);
    fn dos_qqmlapplicationengine_context(vptr: DosQmlApplicationEngine) -> DosQQmlContext;
    fn dos_qqmlapplicationengine_delete(vptr: DosQmlApplicationEngine);

    fn dos_qqmlcontext_setcontextproperty(vptr: DosQQmlContext,
                                          name: DosCStr,
                                          value: DosQVariant);

}

/// Provides an entry point for building QML applications from Rust
pub struct QmlEngine {
    ptr: DosQmlApplicationEngine,
    stored: Vec<QVariant>,
}

impl QmlEngine {
    /// Creates a QML context of a non-headless application
    pub fn new() -> Self {
        unsafe {
            dos_qguiapplication_create();
            QmlEngine {
                ptr: dos_qqmlapplicationengine_create(),
                stored: Vec::new(),
            }
        }
    }

    /// Loads a file as a qml file
    pub fn load_file(&mut self, path: &str) {
        let path_raw = ::std::env::current_dir().unwrap().join(path);
        let path = if cfg!(windows) {
            format!("file:///{}", path_raw.display())
        } else {
            format!("file://{}", path_raw.display())
        };
        unsafe { dos_qqmlapplicationengine_load_url(self.ptr, construct_qurl(&path)) }
    }

    /// Loads qml from a specified url (`file://`, `qrc://`, `http://`)
    pub fn load_url(&mut self, uri: &str) {
        unsafe { dos_qqmlapplicationengine_load_url(self.ptr, construct_qurl(uri)) }
    }

    /// Adds a path to the QML import path
    /// On an "import ModuleName" call QML will additionally search this path for the matching module.
    pub fn add_import_path(&mut self, path: &str) {
        unsafe { dos_qqmlapplicationengine_add_import_path(self.ptr, stoptr(path)) }
    }

    /// Loads a string as a qml file
    pub fn load_data(&mut self, data: &str) {
        unsafe { dos_qqmlapplicationengine_load_data(self.ptr, stoptr(data)) }
    }

    /// Launches the application
    pub fn exec(&mut self) {
        unsafe {
            dos_qguiapplication_exec();
        }
    }
    /// Closes the application
    pub fn quit(&mut self) {
        unsafe {
            dos_qguiapplication_quit();
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
    pub fn set_property(&mut self, name: &str, value: &QVariant) {
        unsafe {
            let context = dos_qqmlapplicationengine_context(self.ptr);
            dos_qqmlcontext_setcontextproperty(context, stoptr(name), get_private_variant(value));
        }
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
            dos_qguiapplication_quit();
            dos_qqmlapplicationengine_delete(self.ptr);
            dos_qguiapplication_delete();
        }
    }
}
