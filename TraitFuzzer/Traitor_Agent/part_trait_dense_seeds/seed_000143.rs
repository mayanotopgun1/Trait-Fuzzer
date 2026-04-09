trait Foo {
    const BLAH: &'static str;
}

trait FooExt: Foo {
    fn is_blah(&self, x: &str) -> bool {
        <Self as Foo>::BLAH == x
    }
}

impl<T> FooExt for T where T: Foo {}

struct Placeholder;

impl Foo for Placeholder {
    const BLAH: &'static str = "hi";
}

fn foo(x: &str) -> impl core::fmt::Debug {
    let placeholder = Placeholder;
    if placeholder.is_blah(x) {
        true
    } else {
        false
    }
}

fn main() {}