trait Foo: Send {}

trait FooExt: Foo {
    type Assoc<'a>: 'a where Self: 'a;

    fn foo_id(&self) -> Self::Assoc<'_>;
}

impl<T: Foo> FooExt for T {
    type Assoc<'a> = &'a T where T: 'a;

    fn foo_id(&self) -> Self::Assoc<'_> {
        self
    }
}

impl Foo for isize {}

trait Bar: Foo {
    fn bar(&self);
}

impl Bar for isize {
    fn bar(&self) {}
}

pub fn main() {
    let x = 42_isize;
    x.bar();
}