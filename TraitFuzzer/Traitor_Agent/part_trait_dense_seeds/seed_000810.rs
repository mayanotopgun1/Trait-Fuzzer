#![feature(impl_trait_in_fn_trait_return)]

trait FooTrait {
    extern "C" fn foo();
}

impl FooTrait for () {
    extern "C" fn foo() {}
}

fn get_foo() -> impl FooTrait {
    ()
}

fn main() {
    let _ = <() as FooTrait>::foo();
    let _ = <() as FooTrait>::foo();
}