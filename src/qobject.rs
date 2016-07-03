use qvariant::*;

#[macro_export]
macro_rules! Q_OBJECT{
($obj:ty :
    signals:
    $(fn $signalname:ident ( $( $signalvar:ident : $signalqtype:ty ),* );)*
    slots:
    $(fn $slotname:ident ( $( $slotvar:ident : $slotqtype:ty ),* );)* ) =>{
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
                    $("$slotname" => {
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
        }
    };
}

pub trait QObjectMacro {
    fn qmeta_slots(&mut self, name: &str, args: Vec<QVariant>);
}

use libc;
#[repr(C)]
struct SignalDefinition {
    name: *const libc::c_char,
    parametersCount: i32,
    parametersMetaTypes: *const i32,
}

#[repr(C)]
struct SignalDefinitions {
    count: i32,
    definitions: *const SignalDefinition,
}

#[repr(C)]
struct SlotDefinition {
    name: *const libc::c_char,
    returnMetaType: i32,
    parametersCount: i32,
    parametersMetaTypes: *const i32,
}

#[repr(C)]
struct SlotDefinitions {
    count: i32,
    definitions: *const SlotDefinition,
}

#[repr(C)]
struct PropertyDefinition {
    name: *const libc::c_char,
    propertyMetaType: i32,
    readSlot: *const libc::c_char,
    writeSlot: *const libc::c_char,
    notifySignal: *const libc::c_char,
}

#[repr(C)]
struct PropertyDefinitions {
    count: i32,
    definitions: *const PropertyDefinition,
}
