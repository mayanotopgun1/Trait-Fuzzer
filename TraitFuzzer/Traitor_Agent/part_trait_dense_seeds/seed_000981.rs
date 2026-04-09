#![crate_type = "lib"]
#![feature(generic_associated_types)]

#[cfg(target_arch = "wasm32")]
mod wasm_non_clash {
    mod a {
        #[link(wasm_import_module = "a")]
        extern "C" {
            pub fn foo();
        }

        trait FooLike<'a> {
            type Output;
            fn call_foo(&self) -> Self::Output;
        }

        impl<'a> FooLike<'a> for () {
            type Output = ();
            fn call_foo(&self) -> Self::Output {
                unsafe { foo() }
                ()
            }
        }

        trait ExtendedFooLike<'a>: FooLike<'a> {
            fn call_foo_twice(&self) -> Self::Output where Self::Output: core::ops::Add<Output = Self::Output> + Copy {
                let x = self.call_foo();
                x + x
            }
        }

        impl<'a, T: FooLike<'a>> ExtendedFooLike<'a> for T where T::Output: core::ops::Add<Output = T::Output> + Copy {}
    }

    mod b {
        #[link(wasm_import_module = "b")]
        extern "C" {
            pub fn foo() -> usize;
        }

        trait FooLike<'a> {
            type Output;
            fn call_foo(&self) -> Self::Output;
        }

        impl<'a> FooLike<'a> for () {
            type Output = usize;
            fn call_foo(&self) -> Self::Output {
                unsafe { foo() }
            }
        }

        trait ExtendedFooLike<'a>: FooLike<'a> {
            fn call_foo_twice(&self) -> Self::Output where Self::Output: core::ops::Add<Output = Self::Output> + Copy {
                let x = self.call_foo();
                x + x
            }
        }

        impl<'a, T: FooLike<'a>> ExtendedFooLike<'a> for T where T::Output: core::ops::Add<Output = T::Output> + Copy {}
    }
}