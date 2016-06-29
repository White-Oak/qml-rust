use libc;

type QQmlApplicationEngine = *const libc::c_void;


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
    // DOS_API DosQQmlContext *DOS_CALL dos_qqmlapplicationengine_context(DosQQmlApplicationEngine *vptr);
    fn dos_qqmlapplicationengine_delete(vptr: QQmlApplicationEngine);
}

pub struct QmlEngine(QQmlApplicationEngine);

impl QmlEngine {
    pub fn new() -> Self {
        unsafe {
            dos_qapplication_create();
            QmlEngine(dos_qqmlapplicationengine_create())
        }
    }

    pub fn load(&self, path: &str) {
        unsafe { dos_qqmlapplicationengine_load(self.0, path.as_ptr() as *const i8) }
    }

    pub fn exec(&self) {
        unsafe {
            dos_qapplication_exec();
        }
    }

    pub fn quit(&self) {
        unsafe {
            dos_qapplication_quit();
        }
    }
}

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
