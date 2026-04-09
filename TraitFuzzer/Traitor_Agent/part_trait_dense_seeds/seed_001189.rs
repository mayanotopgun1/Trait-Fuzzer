trait FooTrait {
    unsafe fn foo();
}

impl FooTrait for () {
    unsafe fn foo() {}
}

#[unsafe(no_mangle)]
extern "C" fn foo() {}

fn main() {}