/// Eases forming of [`QVariantList`](struct.QVariantList.html) ([`QVariant`](struct.QVariant.html) of array).
///
/// To be more precise, macro generates `Vec<QVariant>` which implements `Into<QVariant>` and `Into<QVariantList>`.
/// # Examples
/// ```
/// # #[macro_use] extern crate qml;
/// # use qml::*;
/// # #[allow(unused_variables)]
/// # fn main() {
/// let shortcut: Vec<QVariant> = qvarlist![["John", [2, 2]], ["Ivan", [10, 0]], ["Mary", [0, 1]]];
/// let qvariant: QVariant = shortcut.into();
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
        #[allow(unused_mut)]
        #[allow(dead_code)]
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
///     pub fn simple_receiver(&mut self) -> Option<&QVariant> {
///         // This is a function that also will be a slot
///         None
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
/// // ...
///
/// # fn main() {
/// let qobject = QExample::new(Example, "My name".into());
/// qobject.simple_signal("Hi from Rust!".into());
/// # }
/// ```
#[macro_export]
macro_rules! Q_OBJECT{
    (
        pub $obj:ident as $wrapper:ident{
            signals:
            $(fn $signalname:ident ( $( $signalvar:ident : $signalqtype:ident ),* );)*

            slots:
            $(fn $slotname:ident ( $( $slotvar:ident : $slotqtype:ident ),* );)*

            properties:
            $($propname:ident : $proptype:ident; read: $read_slot:ident, write: $write_slot:ident,
                notify: $notify_sig:ident;)*
            }) =>{
                #[allow(dead_code)]
                pub struct $wrapper{
                    origin: Box<$obj>,
                    ptr: QObject,
                    properties: ::std::collections::HashMap<&'static str, (QVariant, QMetaType)>,
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

                    #[allow(unused_mut)]
                    #[allow(dead_code)]
                    pub fn with_no_props(origin: $obj)-> Box<Self> {
                        unsafe{
                            let mut local = $wrapper{
                                origin: Box::new(origin),
                                ptr: ::std::mem::uninitialized(),
                                properties: ::std::collections::HashMap::new(),
                            };
                            $(local.properties.insert(stringify!($propname), ($proptype::default().into(), $proptype::metatype()));)*
                            let mut local = Box::new(local);
                            let qobj = QObject::new(&mut *local);
                            ::std::ptr::write(&mut local.ptr, qobj);
                            local
                        }
                    }

                    #[allow(unused_mut)]
                    #[allow(dead_code)]
                    pub fn new(origin: $obj, $($propname: $proptype),*) -> Box<Self>{
                        let mut local = Self::with_no_props(origin);
                        $(local.properties.insert(stringify!($propname), ($propname.into(), $proptype::metatype()));)*
                        local
                    }

                    $(
                    #[allow(dead_code)]
                    pub fn $read_slot(&self) -> &QVariant {
                        // println!("Trying to read");
                        &self.properties.get(stringify!($propname)).unwrap().0
                    }

                    #[allow(dead_code)]
                    pub fn $write_slot(&mut self, input: $proptype) {
                        self.properties.insert(stringify!($propname), (input.into(), $proptype::metatype()));
                    })*

                    #[allow(dead_code)]
                    fn threaded<F: FnOnce(&mut $wrapper) + Send + 'static>(&mut self, f: F){
                        let ptr = ::std::sync::atomic::AtomicPtr::new(self);
                        ::std::thread::spawn(move || {
                            f(unsafe { &mut *ptr.load(::std::sync::atomic::Ordering::Relaxed) });
                        });
                    }
                }

                impl QObjectMacro for $wrapper{
                    #[allow(unused_variables)]
                    #[allow(unused_mut)]
                    #[allow(dead_code)]
                    fn qslot_call(&mut self, name: &str, args: Vec<QVariant>) -> Option<&QVariant>{
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
                                self.$slotname ($($slotvar),*)
                            },)*
                            $(stringify!($read_slot) => {
                                Some(self.$read_slot ())
                            },
                            stringify!($write_slot) => {
                                let mut iter = args.into_iter();
                                let next = next_or_panic (iter.next());
                                let property: $proptype = next.into();
                                self.$write_slot (property);
                                None
                            },)*
                            _ => panic!("Unrecognized slot call: {}", name)
                        }
                    }

                    #[allow(unused_mut)]
                    fn qmeta(&self) -> (Vec<(&'static str, i32, Vec<i32>)>,
                    Vec<(&'static str, i32, i32, Vec<i32>)>,
                    Vec<(&'static str, i32, &'static str, &'static str, &'static str)>,
                    &'static str){
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
                        $(
                            signals.push((stringify!($notify_sig), 0, Vec::new()));
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
                        $(
                            slots.push((stringify!($read_slot), $proptype::metatype() as i32, 0, Vec::new()));
                            slots.push((stringify!($write_slot), QMetaType::Void as i32, 1, vec![$proptype::metatype() as i32]));
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

                    fn get_qobj_mut(&mut self) -> &mut QObject{
                        &mut self.ptr
                    }
                }
            };
        }

/// Generates a wrapper for [`QListModel`](struct.QListModel.html) for static typing and easier management.
///
/// Unlike [`Q_LISTMODEL`](macro.Q_LISTMODEL.html) macro, uses `struct` as a data source.
///
/// # Examples
/// ```no_run
/// # #[macro_use] extern crate qml;
/// # use qml::*;
/// Q_LISTMODEL_ITEM!{
///     pub QTestModel<TestModelItem> {
///         name: String,
///         number: i32,
///     }
/// }
///
/// // ...
///
/// # fn main() {
/// let mut qqae = QmlEngine::new();
/// let mut qalm = QTestModel::new();
/// let item1 = TestModelItem {
///     name: "foo".into(),
///     number: 42
/// };
/// let item2 = TestModelItem {
///     name: "bar".into(),
///     number: 23
/// };
/// qalm.append_item(item1);
/// qalm.append_item(item2);
/// // `&QTestModel` implements `Into<QVariant>`
/// qqae.set_and_store_property("listModel", &qalm);
/// qqae.exec();
/// # }
/// ```
#[macro_export]
macro_rules! Q_LISTMODEL_ITEM{
    (pub $wrapper:ident <$wrapper_item:ident> {
        $($rolename:ident : $roletype:ty,)*
    }) => {

        pub struct $wrapper_item {
            $( $rolename : $roletype, )*
        }

        Q_LISTMODEL!{
            pub $wrapper {
                $($rolename : $roletype),*
            }
        }

        impl $wrapper {
            /// Appends a row to this model
            #[allow(unused_mut)]
            pub fn append_item<T>(&mut self, obj :T ) where T: Into<$wrapper_item> {
                let item: $wrapper_item = obj.into();
                let mut vec = Vec::new();
                $(
                    vec.push(item.$rolename.into());
                )*
                self.qalm.append_row(vec.into_iter());
            }

            /// Inserts a row into this model
            #[allow(unused_mut)]
            pub fn insert_item<T>(&mut self, index: usize, obj :T ) where T: Into<$wrapper_item> {
                let item: $wrapper_item = obj.into();
                let mut vec = Vec::new();
                $(
                    vec.push(item.$rolename.into());
                )*
                self.qalm.insert_row(index, vec.into_iter());
            }
        }

    }
}

/// Generates a wrapper for [`QListModel`](struct.QListModel.html) for static typing and easier management.
///
/// Unlike [`Q_LISTMODEL_ITEM`](macro.Q_LISTMODEL_ITEM.html) macro, uses `tuple` as a data source.
///
/// # Examples
/// ```no_run
/// # #[macro_use] extern crate qml;
/// # use qml::*;
/// Q_LISTMODEL!{
///     pub QTestModel {
///         name: String,
///         number: i32
///     }
/// }
///
/// // ...
///
/// # fn main() {
/// let mut qqae = QmlEngine::new();
/// let mut qalm = QTestModel::new();
/// qalm.append_row("John".into(), 42);
/// qalm.append_row("Oak".into(), 505);
/// // `&QTestModel` implements `Into<QVariant>`
/// qqae.set_and_store_property("listModel", &qalm);
///
/// qqae.load_file("examples/listmodel.qml");
/// qalm.set_data(vec![("OMG".into(), 13317), ("HACKED".into(), 228)]);
/// qalm.change_line(0, "Everything's alright".into(), 123);
/// qqae.exec();
/// # }
/// ```
#[macro_export]
macro_rules! Q_LISTMODEL{
            (pub $wrapper:ident{
                $($rolename:ident : $roletype:ty),*
            }) => {
                pub struct $wrapper {
                    qalm: Box<QListModel<'static>>,
                }

                impl $wrapper {
                    pub fn new() -> Self{
                        $wrapper{ qalm: QListModel::new(&[$(stringify!($rolename)),*])}
                    }

                    /// Appends a row to this model
                    #[allow(unused_mut)]
                    pub fn append_row(&mut self, $($rolename : $roletype),*) {
                        let mut vec = Vec::new();
                        $(
                            vec.push($rolename.into());
                        )*
                        self.qalm.append_row(vec.into_iter());
                    }

                    /// Inserts a row into this model
                    #[allow(unused_mut)]
                    pub fn insert_row(&mut self, index: usize, $($rolename : $roletype),*) {
                        let mut vec = Vec::new();
                        $(
                            vec.push($rolename.into());
                        )*
                        self.qalm.insert_row(index, vec.into_iter());
                    }

                    /// Remove a row from this model
                    #[allow(unused_mut)]
                    pub fn remove_row(&mut self, index: usize) {
                        self.qalm.remove_row(index);
                    }

                    /// Gets an accoiated qvariant
                    pub fn get_qvar(&self) -> QVariant{
                        self.qalm.get_qvar()
                    }

                    /// Sets a specified data for this model
                    #[allow(unused_mut)]
                    pub fn set_data(&mut self, vec: Vec<($($roletype,)*)>) {
                        self.qalm.set_data(vec.into_iter()
                        .map(|($($rolename,)*)| {
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

                    /// Sets the line of the data
                    pub fn change_line(&mut self, index: usize, $($rolename : $roletype),*) {
                        let mut vec = Vec::new();
                        $(
                            vec.push($rolename.into());
                        )*
                        self.qalm.change_line(index, vec);
                    }

                    /// Remove all rows from this model
                    #[allow(unused_mut)]
                    pub fn clear(&mut self) {
                        self.qalm.clear();
                    }


                }

                impl<'a, 'b> From<&'a $wrapper> for QVariant {
                    fn from(i: &$wrapper) -> QVariant {
                        i.get_qvar()
                    }
                }
            }
        }

/// Provides definitions for a type that can be used from QML.
///
/// The same macro is used to prepare a type for being used as a normal type or a singleton.
/// The only requirement is that the type in question should provide `Default` implementation.
/// # Examples
/// ```
/// # #[macro_use] extern crate qml;
/// # use qml::*;
///
/// #[derive(Default)]
/// pub struct Test;
///
/// Q_OBJECT!(
/// pub Test as QTest{
///     signals:
///     slots:
///     properties:
///         name: String; read: get_name, write: set_name, notify: name_changed;
/// });
///
/// Q_REGISTERABLE_QML!(QTest: Test as TestRsObject 1=>0, from TestModule);
/// # fn main() {
///
/// # }
/// ```
/// Later on a type that was made registerable can be used in [`Q_REGISTER_QML`](macro.Q_REGISTER_QML!.html)
/// or in [`Q_REGISTER_SINGLETON_QML`](macro.Q_REGISTER_SINGLETON_QML!.html) macros to be used as a type in QML.
#[macro_export]
macro_rules! Q_REGISTERABLE_QML(
            ($wrapper:ident : $origin:ident as $qml:ident $major:expr=>$minor:expr, from $uri:ident) => {
                impl QMLRegisterable for $wrapper{
                    fn qualify_to_register(&self) ->  (i32, i32, &'static str, &'static str) {
                        ($major, $minor, stringify!($uri), stringify!($qml))
                    }

                    fn get_new(&self) -> *mut c_void {
                        let obj = $wrapper::with_no_props($origin::default());
                        let res = Box::into_raw(obj) as *mut c_void;
                        res
                    }

                    fn get_qobj_from_ptr(&self, ptr: *mut c_void) -> *mut QObject {
                        unsafe {
                            let mut obj: Box<$wrapper> = Box::from_raw(ptr as *mut $wrapper);
                            let res = obj.get_qobj_mut() as *mut QObject;
                            ::std::mem::forget(obj);
                            res
                        }
                    }
                }

                impl $wrapper {
                    pub fn get_shallow() -> Self {
                        unsafe {
                            ::std::mem::uninitialized()
                        }
                    }
                }
            }
        );

/// Registers a type as a QML type.
///
/// To use this macro [`Q_REGISTERABLE_QML`](macro.Q_REGISTERABLE_QML!.html) should be used first.
/// # Examples
/// ```
/// # #[macro_use] extern crate qml;
/// # use qml::*;
///
/// #[derive(Default)]
/// pub struct Test;
///
/// Q_OBJECT!(
/// pub Test as QTest{
///     signals:
///     slots:
///     properties:
///         name: String; read: get_name, write: set_name, notify: name_changed;
/// });
///
/// Q_REGISTERABLE_QML!(QTest: Test as TestRsObject 1=>0, from TestModule);
///
/// // ...
///
/// # fn main() {
/// Q_REGISTER_QML!(QTest);
/// # }
/// ```
/// Then in qml:
///
/// ```qml
/// import TestModule 1.0
///
/// TestRsObject{
///     name: "Oak"
/// }
/// ```
#[macro_export]
macro_rules! Q_REGISTER_QML(
                ($wrapper:ident) => {
                    register_qml_type($wrapper::get_shallow());
                }
        );

/// Registers a type as a singleton type in QML.
///
/// To use this macro [`Q_REGISTERABLE_QML`](macro.Q_REGISTERABLE_QML!.html) should be used first.
/// # Examples
/// ```
/// # #[macro_use] extern crate qml;
/// # use qml::*;
///
/// #[derive(Default)]
/// pub struct Test;
///
/// Q_OBJECT!(
/// pub Test as QTest{
///     signals:
///     slots:
///     properties:
///         name: String; read: get_name, write: set_name, notify: name_changed;
/// });
///
/// Q_REGISTERABLE_QML!(QTest: Test as TestRsSingleton 1=>0, from TestModule);
///
/// // ...
///
/// # fn main() {
/// Q_REGISTER_SINGLETON_QML!(QTest);
/// # }
/// ```
/// Then in qml:
///
/// ```qml
/// import TestModule 1.0
///
/// Item {
///     Component.onCompleted: {
///         console.log(TestRsSingleton.name)
///     }
/// }
/// ```
#[macro_export]
macro_rules! Q_REGISTER_SINGLETON_QML(
                ($wrapper:ident) => {
                    register_qml_singleton_type($wrapper::get_shallow());
                }
        );
