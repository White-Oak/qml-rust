use libc;
use std::mem::forget;

use qvariant::*;
use utils::*;
use types::*;
use qmeta::*;

pub struct QObject {
    ptr: DosQObject,
}

extern "C" {

    fn dos_qobject_create(dObjectPointer: *const Box<&QObjectMacro>,
                          metaObject: DosQMetaObject,
                          dObjectCallback: DObjectCallback)
                          -> DosQObject;
    fn dos_qobject_signal_connect(senderVPtr: DosQObject,
                                  signal: *const libc::c_char,
                                  receiverVPtr: DosQObject,
                                  method: *const libc::c_char,
                                  qtype: i32)
                                  -> bool;
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
    AutoConnection = 0,
    /// The slot is invoked immediately when the signal is emitted. The slot is executed in the signalling thread.
    DirectConnection = 1,
    /// The slot is invoked when control returns to the event loop of the receiver's thread. The slot is executed in the receiver's thread.
    QueuedConnection = 2,
    /// Same as Qt::QueuedConnection, except that the signalling thread blocks until the slot returns. This connection must not be used if the receiver lives in the signalling thread, or else the application will deadlock.
    BlockingQueuedConnection = 3,
    /// This is a flag that can be combined with any one of the above connection types, using a bitwise OR. When Qt::UniqueConnection is set, QObject::connect() will fail if the connection already exists (i.e. if the same signal is already connected to the same slot for the same pair of objects). This flag was introduced in Qt 4.6.
    UniqueConnection = 0x80,
}

/// Called when a slot should be executed
/// @param self The pointer to the QObject in the binded language
/// @param slotName The slotName as DosQVariant. It should not be deleted
/// @param argc The number of arguments
/// @param argv An array of DosQVariant pointers. They should not be deleted
type DObjectCallback = extern "C" fn(*const Box<&QObjectMacro>,
                                     DosQVariant,
                                     i32,
                                     *const DosQVariant);

impl QObject {
    pub fn new(obj: &QObjectMacro) -> QObject {
        unsafe {
            extern "C" fn callback(obj: *const Box<&QObjectMacro>,
                                   qvar: DosQVariant,
                                   argc: i32,
                                   argv: *const DosQVariant) {
                println!("CALLBACK HERE");
            }
            let meta = QMeta::new_for_qobject(obj.qmeta());

            let obj = Box::new(obj);
            let res = QObject {
                ptr: dos_qobject_create(&obj as *const Box<&QObjectMacro>,
                                        get_dos_qmeta(&meta),
                                        callback),
            };
            forget(obj);
            forget(meta);
            res
        }
    }
}

pub fn get_qobj_ptr(o: &QObject) -> DosQObject {
    o.ptr
}
