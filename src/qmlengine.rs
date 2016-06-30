use libc;

use qvariant::*;
use types::*;

extern "C" {
    fn dos_qapplication_create();
    fn dos_qapplication_exec();
    fn dos_qapplication_quit();
    fn dos_qapplication_delete();

    fn dos_qqmlapplicationengine_create() -> QQmlApplicationEngine;
    fn dos_qqmlapplicationengine_load(vptr: QQmlApplicationEngine, filename: *const libc::c_char);
    // fn dos_qqmlapplicationengine_load_url(vptr: *mut DosQQmlApplicationEngine, DosQUrl *url);
    // fn dos_qqmlapplicationengine_load_data(vptr: *mut DosQQmlApplicationEngine, const char *data);
    // fn dos_qqmlapplicationengine_add_import_path(vptr: *mut DosQQmlApplicationEngine, const char *path);
    fn dos_qqmlapplicationengine_context(vptr: QQmlApplicationEngine) -> DosQQmlContext;
    fn dos_qqmlapplicationengine_delete(vptr: QQmlApplicationEngine);

    fn dos_qqmlcontext_setcontextproperty(vptr: DosQQmlContext,
                                          name: *const libc::c_char,
                                          value: DosQVariant);

}

/// Provides an entry point for building QML applications from Rust
pub struct QmlEngine(QQmlApplicationEngine);

impl QmlEngine {
    /// Creates a QML context of a non-headless application
    pub fn new() -> Self {
        unsafe {
            dos_qapplication_create();
            QmlEngine(dos_qqmlapplicationengine_create())
        }
    }

    /// Loads a file as a qml file
    pub fn load(&self, path: &str) {
        unsafe { dos_qqmlapplicationengine_load(self.0, stoptr(path)) }
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
    /// Leaks memory as for now
    pub fn set_and_store_property<T: Into<QVariant>>(&self, name: &str, value: T) {
        unsafe {
            let context = dos_qqmlapplicationengine_context(self.0);
            let qvar: QVariant = value.into();
            dos_qqmlcontext_setcontextproperty(context, stoptr(name), get_private_variant(&qvar));
        }
    }

    /// Sets a property for this QML context
    pub fn set_property(&self, name: &str, value: &QVariant) {
        unsafe {
            let context = dos_qqmlapplicationengine_context(self.0);
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
            dos_qapplication_quit();
            dos_qqmlapplicationengine_delete(self.0);
            dos_qapplication_delete();
        }
    }
}
