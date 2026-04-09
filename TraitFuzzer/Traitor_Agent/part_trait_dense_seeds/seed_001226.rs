trait FooTrait {
    fn foo() -> *const i8;
}

impl FooTrait for () {
    fn foo() -> *const i8 {
        b"foo" as *const _ as *const i8
    }
}

trait BarTrait {
    fn bar() -> i32;
}

impl BarTrait for () {
    fn bar() -> i32 {
        *&{(1, 2, 3).1}
    }
}

fn main() {
    assert_eq!(<() as FooTrait>::foo(), b"foo" as *const _ as *const i8);
    assert_eq!(<() as BarTrait>::bar(), 2);
}