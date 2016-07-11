use libc;
use std::mem::forget;
use std::slice::from_raw_parts_mut;

use qvariant::*;
use types::*;
use qmeta::*;

#[doc(hidden)]
/// Contains a pointer to raw Qt object.
#[derive(Debug)]
pub struct QObject {
    ptr: DosQObject,
    qmeta: DosQMetaObject,
}

extern "C" {

    fn dos_qobject_create(dObjectPointer: *mut libc::c_void,
                          metaObject: DosQMetaObject,
                          dObjectCallback: DObjectCallback)
                          -> DosQObject;
    fn dos_qobject_signal_connect(senderVPtr: DosQObject,
                                  signal: *const libc::c_char,
                                  receiverVPtr: DosQObject,
                                  method: *const libc::c_char,
                                  qtype: i32)
                                  -> bool;
    fn dos_qobject_delete(deleted: DosQObject);
    fn dos_qmetaobject_delete(vptr: DosQMetaObject);
}

impl Drop for QObject {
    fn drop(&mut self) {
        unsafe {
            dos_qobject_delete(self.ptr);
            dos_qmetaobject_delete(self.qmeta);
        }
    }
}

macro_rules! QT_connect {
    ($sender:ident, $signal:ident, $receiver:ident, $method:tt) => {{
        unimplemented!()
    }}
}

/// This enum describes the types of connection that can be used between signals and slots.
/// In particular, it determines whether a particular signal is delivered to a slot immediately or queued for delivery at a later time.
pub enum QtConnectionType {
    /// **(Default)** If the receiver lives in the thread that emits the signal, Qt::DirectConnection is used. Otherwise, Qt::QueuedConnection is used. The connection type is determined when the signal is emitted.
    Auto = 0,
    /// The slot is invoked immediately when the signal is emitted. The slot is executed in the signalling thread.
    Direct = 1,
    /// The slot is invoked when control returns to the event loop of the receiver's thread. The slot is executed in the receiver's thread.
    Queued = 2,
    /// Same as Qt::QueuedConnection, except that the signalling thread blocks until the slot returns. This connection must not be used if the receiver lives in the signalling thread, or else the application will deadlock.
    BlockingQueued = 3,
    /// This is a flag that can be combined with any one of the above connection types, using a bitwise OR. When Qt::UniqueConnection is set, QObject::connect() will fail if the connection already exists (i.e. if the same signal is already connected to the same slot for the same pair of objects). This flag was introduced in Qt 4.6.
    Unique = 0x80,
}

/// Called when a slot should be executed
/// @param self The pointer to the `QObject` in the binded language
/// @param slotName The slotName as `DosQVariant`. It should not be deleted
/// @param argc The number of arguments
/// @param argv An array of `DosQVariant` pointers. They should not be deleted
type DObjectCallback = extern "C" fn(*mut libc::c_void, DosQVariant, i32, *mut DosQVariant);

impl QObject {
    pub fn new(obj: &mut QObjectMacro) -> QObject {
        unsafe {
            let qmeta = QMetaDefinition::new(obj.qmeta());
            let meta = QMeta::new_for_qobject(qmeta);

            // println!("Adress of wrapper {:p}", obj);
            let obj = Box::new(obj);

            QObject {
                ptr: dos_qobject_create(Box::into_raw(obj) as *mut libc::c_void,
                                        get_dos_qmeta(&meta),
                                        callback),
                qmeta: get_dos_qmeta(&meta),
            }
        }
    }
}

pub fn get_qobj_ptr(o: &QObject) -> DosQObject {
    o.ptr
}

extern "C" fn callback(obj: *mut libc::c_void,
                       slotName: DosQVariant,
                       argc: i32,
                       argv: *mut DosQVariant) {
    unsafe {
        let mut obj: Box<&mut QObjectMacro> = Box::from_raw(obj as *mut &mut QObjectMacro);
        // println!("Calling adress of wrapper  {:p}", *obj.as_mut());
        let mut slice = from_raw_parts_mut(argv, argc as usize);
        let vec: Vec<QVariant> = slice.iter().skip(1).map(|&dq| dq.into()).collect();
        let slotName: String = new_qvariant(slotName).into();
        // println!("Right before going in... name: {}, argc: {}",
        //  slotName,
        //  argc);
        if let Some(qvar) = obj.qslot_call(&slotName, vec) {
            slice[0] = get_private_variant(qvar);
        }
        forget(obj);
    }
}
