#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

mod m {
    trait Debuggable { type Out; fn get_debug(&self) -> &Self::Out; }

    pub type Foo = impl std::fmt::Debug;
    #[define_opaque(Foo)]
    pub fn foo() -> Foo {
        22_u32
    }

    impl Debuggable for u32 {
        type Out = u32;
        fn get_debug(&self) -> &Self::Out { self }
    }

    pub fn bar() {
        is_send(foo());
    }

    fn is_send<T: Send>(_: T) {}

    trait DebugCheck: Debuggable where Self::Out: std::fmt::Debug { fn check_debug(&self) -> bool { true } }

    impl<T> DebugCheck for T where T: Debuggable, T::Out: std::fmt::Debug {}
}

fn main() {}