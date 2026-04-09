#![feature(rustc_attrs)]
#![feature(fn_align)]

trait Test {
    #[rustc_align(4096)]
    fn foo(&self);

    #[rustc_align(4096)]
    fn foo1(&self);
}

trait TestExtra: Test {
    fn foo_twice(&self) {
        self.foo();
        self.foo();
    }
}

impl<T> TestExtra for T where T: Test {}

fn main() {
    assert_eq!((<dyn Test>::foo as fn(_) as usize & !1) % 4096, 0);
    assert_eq!((<dyn Test>::foo1 as fn(_) as usize & !1) % 4096, 0);
}