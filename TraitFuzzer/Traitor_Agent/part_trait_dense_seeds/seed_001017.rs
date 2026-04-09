#![feature(generic_associated_types)]
#![feature(return_position_impl_trait_in_trait)]

trait Assignable<'a> {
    type Target: 'a;
    fn assign(&mut self, value: &'a str) -> impl core::fmt::Debug;
}

trait DoubleAssign<'a>: Assignable<'a> {
    fn double_assign(&mut self, value1: &'a str, value2: &'a str) -> impl core::fmt::Debug {
        self.assign(value1);
        self.assign(value2)
    }
}

impl<'a, T> DoubleAssign<'a> for T where T: Assignable<'a> {}

impl<'a> Assignable<'a> for String {
    type Target = &'a str;
    fn assign(&mut self, value: &'a str) -> impl core::fmt::Debug {
        *self = value.to_string();
        "Assigned"
    }
}

macro_rules! do_block {
    ($block:block) => {
        $block
    };
}

fn main() {
    let mut s = String::new();
    let result = do_block!({
        s.double_assign("first", "it works!");
        &s
    });
    assert_eq!(result, "it works!");
}