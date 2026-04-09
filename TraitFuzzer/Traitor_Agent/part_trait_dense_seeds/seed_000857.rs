trait Foo {
    fn foo(&self);
}

trait FooDebug: Foo {
    #[track_caller]
    fn debug_foo(&self) {}
}

impl<T: ?Sized + Foo> FooDebug for T {}

struct Bar;
impl Foo for Bar {
    fn foo(&self) {}
}

struct Baz;
impl Foo for Baz {
    fn foo(&self) {}
}

fn main() {

    let f: fn(&dyn Foo) = |x| x.foo();
    f(&Bar);

    let v: &dyn FooDebug = &Baz as _;
    v.debug_foo();
}