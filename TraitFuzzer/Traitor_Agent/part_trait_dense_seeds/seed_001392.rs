#[diagnostic::non_existing_attribute]

pub trait Bar {
    fn bar_method(&self);
}

impl Bar for Foo {
    fn bar_method(&self) {}
}

#[diagnostic::non_existing_attribute(with_option = "foo")]

struct Foo;

fn main() {
    let foo = Foo;
    foo.bar_method();
}