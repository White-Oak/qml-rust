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

    fn dos_qobject_create(dObjectPointer: *const Box<QObjectMacro>,
                          metaObject: DosQMetaObject,
                          dObjectCallback: DObjectCallback)
                          -> DosQObject;
}


/// Called when a slot should be executed
/// @param self The pointer to the QObject in the binded language
/// @param slotName The slotName as DosQVariant. It should not be deleted
/// @param argc The number of arguments
/// @param argv An array of DosQVariant pointers. They should not be deleted
type DObjectCallback = extern "C" fn(*const Box<QObjectMacro>,
                                     DosQVariant,
                                     i32,
                                     *const DosQVariant);

impl QObject {
    pub fn new(obj: Box<QObjectMacro>) -> QObject {
        unsafe {
            extern "C" fn callback(obj: *const Box<QObjectMacro>,
                                   qvar: DosQVariant,
                                   argc: i32,
                                   argv: *const DosQVariant) {
                println!("CALLBACK HERE");
            }
            let meta = QMeta::new_for_qobject(obj.qmeta());

            let res = QObject {
                ptr: dos_qobject_create(&obj as *const Box<QObjectMacro>,
                                        get_dos_qmeta(&meta),
                                        callback),
            };
            forget(obj);
            forget(meta);
            res
        }
    }
}
