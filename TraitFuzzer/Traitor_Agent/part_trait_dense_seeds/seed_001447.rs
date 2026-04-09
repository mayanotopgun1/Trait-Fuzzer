#[no_mangle]
extern "C" fn foo() {}

trait Foo {
    fn execute();
}

impl Foo for () {
    fn execute() {
        foo()
    }
}

fn main() {
    <() as Foo>::execute();
}