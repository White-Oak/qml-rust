/// Eases forming of `QVariantLists` ([`QVariant`](struct.QVariant.html) of array).
///
/// To be more precise, macro generates Vec<QVariant> which implements Into<QVariant>.
/// # Examples
/// ```
/// # #[macro_use] extern crate qml;
/// # use qml::*;
/// # fn main() {
/// let shortcut: QVariant = qvarlist![["John", [2, 2]], ["Ivan", [10, 0]], ["Mary", [0, 1]]].into();
/// # }
/// ```
#[macro_export]
macro_rules! qvarlist{
    (__ $v:ident, [$($inside:tt)*], $($rest:tt)*) => {
        $v.push(qvarlist!($($inside)*).into());
        qvarlist!(__ $v, $($rest)*);
    };
    (__ $v:ident, [$($inside:tt)*]) => {
        $v.push(qvarlist!($($inside)*).into());
    };
    (__ $v:ident, $varname:expr, $($rest:tt)*) => {
        $v.push($varname.into());
        qvarlist!(__ $v, $($rest)*);
    };
    (__ $v:ident, $varname:expr) => {
        $v.push($varname.into());
    };
    ($($rest:tt)*) => {{
        let mut v: Vec<QVariant> = Vec::new();
        qvarlist!(__ v, $($rest)*);
        v
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __gen_signals{
    (fn $signalname:ident ( $( $signalvar:ident : $signalqtype:ident ),* ); $($rest:tt)*) =>{
        pub fn $signalname(&self, $( $signalvar: $signalqtype ),*){
            let mut vec: Vec<QVariant> = Vec::new();
            $(
                let $signalvar: $signalqtype = $signalvar;
                vec.push($signalvar.into());
            )*
            emit_signal(self, stringify!($signalname), vec);
        }

        __gen_signals!($($rest)*);
    };
    () => ();
}

/// Marks the structure to be able to be used in Qt meta-object system.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate qml;
/// # use qml::*;
/// pub struct Example;
///
/// impl Example {
///     pub fn simple_receiver(&mut self) {
///         // This is a function that also will be a slot
///     }
/// }
///
/// Q_OBJECT!(
/// pub Example as QExample{
///     signals:
///         fn simple_signal(s: String);
///     slots:
///         fn simple_receiver();
///     properties:
///         name: String; read: get_name, write: set_name, notify: name_changed;
/// });
///
/// ...
///
/// # fn main() {
/// let mut qqae = QmlEngine::new();
/// let mut qobject = QExample::new(Example);
/// qobject.simple_signal("Hi from Rust!".into());
/// # }
/// ```
#[macro_export]
macro_rules! Q_OBJECT{
(
    pub $obj:ty as $wrapper:ident{
        signals:
        $(fn $signalname:ident ( $( $signalvar:ident : $signalqtype:ident ),* );)*

        slots:
        $(fn $slotname:ident ( $( $slotvar:ident : $slotqtype:ident ),* );)*

        properties:
        $($propname:ident : $proptype:ident; read: $read_slot:ident, write: $write_slot:ident,
             notify: $notify_sig:ident;)*
    }) =>{

        fn get_atomic_ptr<T>(o: &mut T) -> ::std::sync::atomic::AtomicPtr<T> {
            ::std::sync::atomic::AtomicPtr::new(o as *mut T)
        }

        fn load_borrow<T>(ptr: ::std::sync::atomic::AtomicPtr<T>) -> &'static mut T {
            unsafe { &mut *ptr.load(::std::sync::atomic::Ordering::Relaxed) }
        }

        pub struct $wrapper{
            origin: Box<$obj>,
            ptr: QObject,
            $($propname: $proptype,)*
        }

        impl ::std::ops::Deref for $wrapper {
            type Target = $obj;

            fn deref(&self) -> &$obj {
                let ref b: Box<$obj> = self.origin;
                b.as_ref()
            }
        }

        impl ::std::ops::DerefMut for $wrapper {
            fn deref_mut<'a>(&'a mut self) -> &'a mut $obj {
                self.origin.as_mut()
            }
        }

        impl $wrapper{
            __gen_signals!($(fn $signalname ( $( $signalvar : $signalqtype ),* );)*
            $(fn $notify_sig ();)*);

            pub fn new(origin: $obj, $($propname: $proptype),*) -> Box<Self>{
                unsafe{
                    let mut local = $wrapper{
                        origin: Box::new(origin),
                        ptr: ::std::mem::uninitialized(),
                        $($propname: $propname,)*
                    };
                    let mut local = Box::new(local);
                    let qobj = QObject::new(&mut *local);
                    ::std::ptr::write(&mut local.ptr, qobj);
                    local
                }
            }

            $(pub fn $read_slot(&self) -> $proptype {
                self.$propname.clone()
            }

            pub fn $write_slot(&mut self, input: $proptype) {
                self.$propname = input
            })*
        }

        impl QObjectMacro for $wrapper{
            fn qslot_call(&mut self, name: &str, args: Vec<QVariant>) {
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
                    $(stringify!($read_slot) => {
                        let sending = self.$read_slot ();
                    },
                    stringify!($write_slot) => {
                        let mut iter = args.into_iter();
                        let next = next_or_panic (iter.next());
                        let property: $proptype = next.into();
                        self.$write_slot (property);
                    },)*
                    _ => panic!("Unrecognized slot call: {}", name)
                }
            }

            fn qmeta(&self) -> (Vec<(&'static str, i32, Vec<i32>)>,
                                Vec<(&'static str, i32, i32, Vec<i32>)>,
                                Vec<(&'static str, i32, &'static str, &'static str, &'static str)>,
                                &'static str){
                use qml::qtypes::*;
                let mut signals = Vec::new();
                $(
                    let mut argc = 0;
                    let mut mttypes = Vec::new();
                    $(
                        argc += 1;
                        mttypes.push($signalqtype::metatype() as i32);
                    )*
                    signals.push((stringify!($signalname), argc, mttypes));
                )*
                let mut slots = Vec::new();
                $(
                    let mut argc = 0;
                    let mut mttypes = Vec::new();
                    $(
                        argc += 1;
                        mttypes.push($slotqtype::metatype() as i32);
                    )*
                    slots.push((stringify!($slotname), 43, argc, mttypes));
                )*
                let mut props: Vec<(&'static str, i32, &'static str, &'static str, &'static str)> = Vec::new();
                $(
                    props.push((stringify!($propname), $proptype::metatype() as i32, stringify!($read_slot),
                               stringify!($write_slot), stringify!($notify_sig)));
                )*
                (signals, slots, props, stringify!($obj))
            }

            fn get_qobj(&self) -> &QObject{
                &self.ptr
            }
        }
    };
}


// #[macro_export]
// macro_rules! __listmodel_helper{
//     (()$roletype:ident),*) => {
//         ($(v.next().unwrap().into()),*)
//     }
// }

/// Generates a wrapper for [`QListModel`](struct.QListModel.html) for static typing and easier management.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate qml;
/// # use qml::*;
/// Q_LISTMODEL!{
///     pub QTestModel {
///         name: &str,
///         number: i32,
///     }
/// }
///
/// ...
///
/// # fn main() {
/// let mut qqae = QmlEngine::new();
/// let mut qalm = QTestModel::new();
/// qalm.insert_row("John", 42);
/// qalm.insert_row("Oak", 505);
/// // `&QTestModel` implements `Into<QVariant>`
/// qqae.set_and_store_property("listModel", &qalm);
///
/// qqae.load_file("examples/listmodel.qml");
/// qalm.set_data(vec![("OMG", 13317), ("HACKED", 228)]);
/// qqae.exec();
/// # }
/// ```
#[macro_export]
macro_rules! Q_LISTMODEL{
    (pub $wrapper:ident{
        $($rolename:ident : $roletype:ty,)*
    }) => {
        pub struct $wrapper {
            qalm: Box<QListModel<'static>>,
        }

        impl $wrapper {
            pub fn new() -> Self{
                $wrapper{ qalm: QListModel::new(&[$(stringify!($rolename)),*])}
            }

            /// Inserts a row into this model
            pub fn insert_row(&mut self, $($rolename : $roletype),*) {
                let mut vec = Vec::new();
                $(
                    vec.push($rolename.into());
                )*
                self.qalm.insert_row(vec.into_iter());
            }

            /// Gets an accoiated qvariant
            pub fn get_qvar(&self) -> QVariant{
                self.qalm.get_qvar()
            }

            /// Sets a specified data for this model
            pub fn set_data(&mut self, vec: Vec<($($roletype),*)>) {
                self.qalm.set_data(vec.into_iter()
                .map(|($($rolename),*)| {
                    let mut vec = Vec::new();
                    $(
                        vec.push($rolename.into());
                    )*
                    vec
                }).collect::<Vec<Vec<QVariant>>>())
            }

            /// View contents of this model as a slice of rows of QVariants
            pub fn view_raw_data(&self) -> &[Vec<QVariant>]{
                self.qalm.view_data()
            }

            /// View contents of this model as a row
            pub fn view_data(&self) -> Vec<($($roletype),*)>{
                let view = self.qalm.view_data();
                view.into_iter().map(|v| {
                    let mut v = v.iter();
                    $(
                        let $rolename = v.next().unwrap().into();
                    )*
                    ($($rolename),*)
                }).collect()
            }
        }

        impl<'a, 'b> From<&'a $wrapper> for QVariant {
            fn from(i: &$wrapper) -> QVariant {
                i.get_qvar()
            }
        }
    }
}
