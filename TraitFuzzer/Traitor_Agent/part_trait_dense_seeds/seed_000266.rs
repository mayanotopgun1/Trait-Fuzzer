#![feature(type_alias_impl_trait)]

trait FooTrait {
    type Out;
    fn foo(&self, x: bool, y: bool) -> Self::Out;
}

impl FooTrait for () {
    type Out = u32;

    fn foo(&self, x: bool, y: bool) -> Self::Out {
        match (x, y) {
            (false, _) => 0,
            (_, false) => 1,
            (true, true) => 2,
        }
    }
}

fn main() {
    let unit = ();
    assert_eq!(unit.foo(false, true), 0);
    assert_eq!(unit.foo(false, false), 0);
    assert_eq!(unit.foo(true, false), 1);
    assert_eq!(unit.foo(true, true), 2);
}