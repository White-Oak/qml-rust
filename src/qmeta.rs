use std::mem::forget;
use libc;

use qvariant::*;
use utils::*;
use types::*;

#[macro_export]
macro_rules! Q_OBJECT{
($obj:ty :
    signals:
    $(fn $signalname:ident ( $( $signalvar:ident : $signalqtype:ident ),* );)*
    slots:
    $(fn $slotname:ident ( $( $slotvar:ident : $slotqtype:ident ),* );)* ) =>{
        impl $obj{
            $(fn $signalname(&self, $( $signalvar: $signalqtype ),*){
                let mut vec: Vec<QVariant> = Vec::new();
                $(
                    let $signalvar: $signalqtype = $signalvar;
                    vec.push($signalvar.into());
                )*
            })*
        }

        impl QObjectMacro for $obj{
            fn qmeta_slots(&mut self, name: &str, args: Vec<QVariant>) {
                fn next_or_panic(qt: Option<QVariant>) -> QVariant{
                    if let Some(o) = qt {
                        o
                    }else {
                        panic!("Not enough parameters to call a slot")
                    }
                }
                match name {
                    $(stringify!($slotname) => {
                        let mut iter = args.into_iter();
                        $(
                            let next = next_or_panic (iter.next());
                            let $slotvar: $slotqtype = next.into();
                        )*
                        self.$slotname ($($slotvar),*);
                    },)*
                    _ => ()
                }
            }

            fn qmeta(&self) -> QMetaDefinition{
                let mut signals = Vec::new();
                $(
                    let mut argc = 0;
                    let mut mttypes = Vec::new();
                    $(
                        argc += 1;
                        mttypes.push($signalqtype::metatype());
                    )*
                    signals.push((stringify!($signalname), argc, mttypes));
                )*
                let mut slots = Vec::new();
                $(
                    let $slotname = ();
                    let mut argc = 0;
                    let mut mttypes = Vec::new();
                    $(
                        argc += 1;
                        mttypes.push($slotqtype::metatype());
                    )*
                    slots.push((stringify!($slotname), 43, argc, mttypes));
                )*
                QMetaDefinition::new(signals, slots, stringify!($obj))
            }
        }
    };
}

extern "C" {
    fn dos_qmetaobject_create(superClassMetaObject: DosQMetaObject,
                              className: *const libc::c_char,
                              signalDefinitions: *const SignalDefinitions,
                              slotDefinitions: *const SlotDefinitions,
                              propertyDefinitions: *const PropertyDefinitions)
                              -> DosQMetaObject;
    fn dos_qobject_qmetaobject() -> DosQMetaObject;
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

pub trait QMetaType<T> {
    fn metatype() -> i32;
}

impl QMetaType<i32> for i32 {
    fn metatype() -> i32 {
        2
    }
}

#[derive(Debug)]
pub struct QMetaDefinition {
    sig_defs: SignalDefinitions,
    slot_defs: SlotDefinitions,
    prop_defs: PropertyDefinitions,
    name: &'static str,
}

impl QMetaDefinition {
    pub fn new(signals: Vec<(&str, i32, Vec<i32>)>,
               slots: Vec<(&str, i32, i32, Vec<i32>)>,
               name: &'static str)
               -> Self {
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
        QMetaDefinition {
            sig_defs: sig_defs,
            slot_defs: slot_defs,
            prop_defs: PropertyDefinitions::default(),
            name: name,
        }
    }
}

pub trait QObjectMacro {
    fn qmeta_slots(&mut self, name: &str, args: Vec<QVariant>);
    fn qmeta(&self) -> QMetaDefinition;
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

impl Default for PropertyDefinitions {
    fn default() -> Self {
        let vec = Vec::new();
        let res = PropertyDefinitions {
            count: 0,
            definitions: vec.as_ptr(),
        };
        forget(vec);
        res
    }
}
