trait FooTrait { const FOO: &'static [u8]; }
impl FooTrait for () { const FOO: &'static [u8] = b"foo"; }

trait BarTrait { const BAR: i32; }
impl BarTrait for () { const BAR: i32 = 2; }

trait FooBarTrait: FooTrait + BarTrait {
    fn foo_bar() -> *const i8 {
        <Self as FooTrait>::FOO as *const _ as *const i8
    }
}

impl FooBarTrait for () {}

const fn bar() -> i32 {
    *&{(1, 2, 3).1}
}

fn main() {
    assert_eq!(<() as FooBarTrait>::foo_bar(), b"foo" as *const _ as *const i8);
    assert_eq!(bar(), <() as BarTrait>::BAR);
}