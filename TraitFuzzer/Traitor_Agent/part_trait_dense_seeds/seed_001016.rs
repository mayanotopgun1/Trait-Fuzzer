#![feature(generic_associated_types)]

trait Assignable<'a> {
    type Target: 'a;
    fn assign(&mut self, value: &'a str);
}

trait DoubleAssign<'a>: Assignable<'a> {
    fn double_assign(&mut self, value1: &'a str, value2: &'a str) {
        self.assign(value1);
        self.assign(value2);
    }
}

impl<'a, T> DoubleAssign<'a> for T where T: Assignable<'a> {}

impl<'a> Assignable<'a> for String {
    type Target = &'a str;
    fn assign(&mut self, value: &'a str) {
        *self = value.to_string();
    }
}

macro_rules! do_block {
    ($block:block) => {
        $block
    };
}

fn main() {
    let mut s = String::new();
    do_block!({ s.double_assign("first", "it works!"); });
    assert_eq!(s, "it works!");
}