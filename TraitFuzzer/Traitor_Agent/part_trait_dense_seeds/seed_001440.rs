#![feature(associated_type_defaults)]

trait Tr {
    type A = Vec<Self::B>;
    type B = Box<Self::A>;

    fn f();
}

trait TrExt: Tr where Self::A: Default {
    fn g(&self) -> Self::A {
        Self::f();
        Default::default()
    }
}

impl<T> TrExt for T where T: Tr, <T as Tr>::A: Default {}

impl Tr for u8 {
    type A = u8;

    fn f() {
        let _: Self::A = 0u8;
        let _: Self::B = Box::new(0u8);
    }
}

impl Tr for String {
    type B = ();

    fn f() {
        let _: Self::A = Vec::<()>::new();
        let _: Self::B = ();
    }
}

impl Tr for () {
    type A = Vec<()>;
    type B = u8;

    fn f() {
        let _: Self::A = Vec::<()>::new();
        let _: Self::B = 0u8;
    }
}

fn main() {
    let _: <u8 as Tr>::A = 0u8;
    let _: <u8 as Tr>::B = Box::new(0u8);

    let _: <String as Tr>::A = Vec::<()>::new();
    let _: <String as Tr>::B = ();

    let _: <() as Tr>::A = Vec::<()>::new();
    let _: <() as Tr>::B = 0u8;
}