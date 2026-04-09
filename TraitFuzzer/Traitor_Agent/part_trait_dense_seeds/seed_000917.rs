#[cfg(foo)]
macro_rules! foo { () => (1) }

#[cfg(not(foo))]
macro_rules! foo { () => (2) }

trait FooMacro {
    fn get_value() -> i32;
}

impl FooMacro for () {
    fn get_value() -> i32 {
        foo!()
    }
}

pub fn main() {
    assert_eq!(<() as FooMacro>::get_value(), 1);
}