trait Bar {
    type Assoc<const N: usize>;
}

trait Foo: Bar {
    fn foo(&self) -> Self::Assoc<3>;
}

trait FooExt: Foo {
    fn foo_ext(&self) -> Self::Assoc<3> where Self::Assoc<3>: core::fmt::Debug {
        self.foo()
    }
}

impl<S> FooExt for S where S: Foo {}

impl Bar for () {
    type Assoc<const N: usize> = [(); N];
}

impl Foo for () {
    fn foo(&self) -> Self::Assoc<3> {
        [(); 3]
    }
}

fn main() {
    assert_eq!(().foo_ext(), [(); 3]);
}