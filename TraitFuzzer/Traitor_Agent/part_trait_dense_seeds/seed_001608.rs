#![feature(macro_derive, associated_type_defaults)]

#[macro_export]
macro_rules! MyExportedDerive {
    derive() { $($body:tt)* } => {
        println!("MyExportedDerive: body={:?}", stringify!($($body)*));
    };
    { $($args:tt)* } => {
        println!("MyExportedDerive!({:?})", stringify!($($args)*));
    };
}

macro_rules! MyLocalDerive {
    derive() { $($body:tt)* } => {
        println!("MyLocalDerive: body={:?}", stringify!($($body)*));
    };
    { $($args:tt)* } => {
        println!("MyLocalDerive!({:?})", stringify!($($args)*));
    };
}

trait MyTrait {
    fn name() -> &'static str;
}

trait MyTraitExt: MyTrait {
    fn name_id(&self) -> &'static str where Self: Sized {
        Self::name()
    }
}

impl<T: MyTrait> MyTraitExt for T {}

macro_rules! MyTrait {
    derive() { struct $name:ident; } => {
        impl MyTrait for $name {
            fn name() -> &'static str {
                stringify!($name)
            }
        }
    };
}

#[derive(MyTrait)]
struct MyGlobalType;

fn main() {
    #[derive(crate::MyExportedDerive)]
    struct _S1;
    #[derive(crate::MyExportedDerive, crate::MyExportedDerive)]
    struct _Twice1;

    crate::MyExportedDerive!();
    crate::MyExportedDerive!(invoked, arguments);

    #[derive(MyExportedDerive)]
    struct _S2;
    #[derive(MyExportedDerive, MyExportedDerive)]
    struct _Twice2;

    MyExportedDerive!();
    MyExportedDerive!(invoked, arguments);

    #[derive(MyLocalDerive)]
    struct _S3;
    #[derive(MyLocalDerive, MyLocalDerive)]
    struct _Twice3;

    MyLocalDerive!();
    MyLocalDerive!(invoked, arguments);

    #[derive(MyTrait)]
    struct MyLocalType;

    println!("MyGlobalType::name_id(): {}", MyGlobalType.name_id());
    println!("MyLocalType::name_id(): {}", MyLocalType.name_id());
}