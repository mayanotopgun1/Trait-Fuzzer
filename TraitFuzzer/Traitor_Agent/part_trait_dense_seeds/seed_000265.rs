trait FooTrait {
    fn foo(&self, x: bool, y: bool) -> u32;
}

impl FooTrait for () {
    fn foo(&self, x: bool, y: bool) -> u32 {
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