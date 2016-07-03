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
