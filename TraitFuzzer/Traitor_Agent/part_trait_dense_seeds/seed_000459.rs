#![feature(type_alias_impl_trait)]

trait Operate {
    type Result;
    fn operate(&self, a: i32, b: i32) -> Self::Result;
}

impl Operate for &'static str {
    type Result = i32;
    fn operate(&self, a: i32, b: i32) -> Self::Result {
        match *self {
            "+" => ::std::ops::Add::add(a, b),
            "-" => ::std::ops::Sub::sub(a, b),
            "<" => (a < b) as i32,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let operator: &'static str = "+";
    let result = operator.operate(5, 5);
    let _: i32 = result;
}