trait Foo {
    type Item<'a>: ?Sized where Self: 'a;
}

impl Foo for () {
    type Item<'a> = dyn 'a + Send;
}

trait FooExt: Foo {
    fn create_foo(&self, x: &str, y: &str) -> Box<Self>;
}

impl FooExt for () {
    fn create_foo(&self, _x: &str, _y: &str) -> Box<Self> {
        Box::new(())
    }
}

trait FooMethod {
    fn foo_method(&self);
}

impl FooMethod for () {
    fn foo_method(&self) {}
}

fn main() {
    let factory = ();
    let foo_instance = factory.create_foo("", "");
    foo_instance.foo_method();
}