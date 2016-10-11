extern crate qml;

use std::cell::UnsafeCell;
use std::marker::Sync;
use std::thread;
use std::time;
use std::sync::Arc;

use qml::*;

struct NotThreadSafe<T> {
    value: UnsafeCell<T>,
}

// The unsafe code is needed to make QmlEngine multithreadedd, since I am not sure, whether should this be in a library.
// The simple example would be:
//fn main() {
//    let mut qqae = QmlEngine::new();
//    qqae.load_url("qrc:///qml/resources.qml");
//    qqae.exec();
//}


unsafe impl<T> Sync for NotThreadSafe<T> {}
unsafe impl<T> Send for NotThreadSafe<T> {}
fn main() {
    let mut qqae = QmlEngine::new();
    qqae.load_url("qrc:///qml/resources.qml");
    let nts = NotThreadSafe { value: UnsafeCell::new(qqae) };

    let arc = Arc::new(nts);
    let current_arc = arc.clone();
    thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(2));
        unsafe {
            (&mut (*current_arc.value.get())).quit();
        }
    });
    unsafe {
        (&mut (*arc.value.get())).exec();
    }
}
