#![feature(type_alias_impl_trait)]
#![feature(linkage)]

pub mod foo {
    #[linkage = "weak"]
    #[no_mangle]
    pub extern "C" fn FOO() -> i32 {
        0
    }
}

mod bar {
    extern "C" {
        fn FOO() -> i32;
    }

    trait FooTrait {
        type Out: core::fmt::Debug;
        unsafe fn call_foo(&self) -> Self::Out;
    }

    impl FooTrait for () {
        type Out = i32;
        unsafe fn call_foo(&self) -> Self::Out {
            FOO()
        }
    }

    trait FooExt: FooTrait where Self::Out: core::ops::Add<Output = Self::Out> + Copy {
        unsafe fn double_call_foo(&self) -> Self::Out {
            let x = self.call_foo();
            x + x
        }
    }

    impl<T> FooExt for T where T: FooTrait, T::Out: core::ops::Add<Output = T::Out> + Copy {}

    pub fn bar() -> i32 {
        let dummy: () = ();
        unsafe { dummy.double_call_foo() }
    }
}

fn main() {
    bar::bar();
}