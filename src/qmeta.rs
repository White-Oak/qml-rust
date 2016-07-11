use std::mem::forget;
use libc;

use qvariant::*;
use utils::*;
use types::*;
use qobject::*;

extern "C" {
    fn dos_qmetaobject_create(superClassMetaObject: DosQMetaObject,
                              className: *const libc::c_char,
                              signalDefinitions: *const SignalDefinitions,
                              slotDefinitions: *const SlotDefinitions,
                              propertyDefinitions: *const PropertyDefinitions)
                              -> DosQMetaObject;
    fn dos_qobject_qmetaobject() -> DosQMetaObject;
    fn dos_qobject_signal_emit(vptr: DosQObject,
                               name: *const libc::c_char,
                               parametersCount: i32,
                               parameters: *const DosQVariant);
}

#[doc(hidden)]
/// Used by [`Q_OBJECT`](macro.Q_OBJECT!.html) macro to send signals to Qt.
pub fn emit_signal(obj: &QObjectMacro, signalname: &str, args: Vec<QVariant>) {
    let vec: Vec<DosQVariant> = args.iter()
        .map(|qvar| get_private_variant(qvar))
        .collect();
    forget(args);
    unsafe {
        println!("about to send signal");
        dos_qobject_signal_emit(get_qobj_ptr(obj.get_qobj()),
                                stoptr(signalname),
                                vec.len() as i32,
                                vec.as_ptr())
    }
}

pub struct QMeta {
    ptr: DosQMetaObject,
}

pub fn get_dos_qmeta(meta: &QMeta) -> DosQMetaObject {
    meta.ptr
}

impl QMeta {
    pub fn new_for_qobject(def: QMetaDefinition) -> QMeta {
        unsafe {
            let meta_obj = dos_qobject_qmetaobject();
            let dos_meta = dos_qmetaobject_create(meta_obj,
                                                  stoptr(def.name),
                                                  &def.sig_defs as *const SignalDefinitions,
                                                  &def.slot_defs as *const SlotDefinitions,
                                                  &def.prop_defs as *const PropertyDefinitions);
            QMeta { ptr: dos_meta }
        }
    }
}

#[derive(Debug)]
pub struct QMetaDefinition {
    sig_defs: SignalDefinitions,
    slot_defs: SlotDefinitions,
    prop_defs: PropertyDefinitions,
    name: &'static str,
}

pub fn get_qmetadef_name(o: &QMetaDefinition) -> &'static str {
    o.name
}

pub type QMetaDef = (Vec<(&'static str, i32, Vec<i32>)>,
                     Vec<(&'static str, i32, i32, Vec<i32>)>,
                     Vec<(&'static str, i32, &'static str, &'static str, &'static str)>,
                     &'static str);
impl QMetaDefinition {
    pub fn new(input: QMetaDef) -> Self {
        let (signals, slots, props, name) = input;
        let signals: Vec<SignalDefinition> = signals.into_iter()
            .map(|(s, argc, types)| {
                let def = SignalDefinition {
                    name: stoptr(s),
                    parametersCount: argc,
                    parametersMetaTypes: types.as_ptr(),
                };
                forget(types);
                def
            })
            .collect();
        let sig_defs = SignalDefinitions {
            count: signals.len() as i32,
            definitions: signals.as_ptr(),
        };
        forget(signals);

        let slots: Vec<SlotDefinition> = slots.into_iter()
            .map(|(s, ret_type, argc, types)| {
                let def = SlotDefinition {
                    name: stoptr(s),
                    returnMetaType: ret_type,
                    parametersCount: argc,
                    parametersMetaTypes: types.as_ptr(),
                };
                forget(types);
                def
            })
            .collect();
        let slot_defs = SlotDefinitions {
            count: slots.len() as i32,
            definitions: slots.as_ptr(),
        };
        forget(slots);

        let props: Vec<PropertyDefinition> = props.into_iter()
            .map(|(name, propertyMetaType, readSlot, writeSlot, notifySignal)| {
                PropertyDefinition {
                    name: stoptr(name),
                    propertyMetaType: propertyMetaType,
                    readSlot: stoptr(readSlot),
                    writeSlot: stoptr(writeSlot),
                    notifySignal: stoptr(notifySignal),
                }
            })
            .collect();
        let prop_defs = PropertyDefinitions {
            count: props.len() as i32,
            definitions: props.as_ptr(),
        };
        forget(props);
        QMetaDefinition {
            sig_defs: sig_defs,
            slot_defs: slot_defs,
            prop_defs: prop_defs,
            name: name,
        }
    }
}

#[doc(hidden)]
// Provides `qml-rust` with the neccessary information and an ability to callback slots.
pub trait QObjectMacro {
    fn qslot_call(&mut self, name: &str, args: Vec<QVariant>) -> Option<&QVariant>;
    fn qmeta(&self) -> QMetaDef;
    fn get_qobj(&self) -> &QObject;
}

#[derive(Debug)]
#[repr(C)]
struct SignalDefinition {
    name: *const libc::c_char,
    parametersCount: i32,
    parametersMetaTypes: *const i32,
}

#[derive(Debug)]
#[repr(C)]
struct SignalDefinitions {
    count: i32,
    definitions: *const SignalDefinition,
}

#[derive(Debug)]
#[repr(C)]
struct SlotDefinition {
    name: *const libc::c_char,
    returnMetaType: i32,
    parametersCount: i32,
    parametersMetaTypes: *const i32,
}

#[derive(Debug)]
#[repr(C)]
struct SlotDefinitions {
    count: i32,
    definitions: *const SlotDefinition,
}

#[derive(Debug)]
#[repr(C)]
struct PropertyDefinition {
    name: *const libc::c_char,
    propertyMetaType: i32,
    readSlot: *const libc::c_char,
    writeSlot: *const libc::c_char,
    notifySignal: *const libc::c_char,
}

#[derive(Debug)]
#[repr(C)]
struct PropertyDefinitions {
    count: i32,
    definitions: *const PropertyDefinition,
}
