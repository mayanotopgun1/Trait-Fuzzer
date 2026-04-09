trait FooTrait {
    extern "C" fn foo();
}

impl FooTrait for () {
    extern "C" fn foo() {}
}

fn main() {
    let _ = <() as FooTrait>::foo;
    let _ = <() as FooTrait>::foo;
}