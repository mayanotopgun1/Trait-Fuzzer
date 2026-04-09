#![feature(type_alias_impl_trait)]

use std::borrow::Cow;

trait TestTrait<'a> {
    type Value;
    fn test_value(&self) -> Self::Value;
}

impl<'a> TestTrait<'a> for Test<'a> {
    type Value = Cow<'a, str>;
    fn test_value(&self) -> Self::Value {
        match self {
            Test::Int(_) => "Integer".into(),
            Test::Array(array) => format!("Array of length {}", array.len()).into(),
        }
    }
}

#[derive(Clone)]
enum Test<'a> {
    Int(u8),
    Array(Cow<'a, [Test<'a>]>),
}

fn main() {
    let test1 = Test::Int(42);
    let test2 = Test::Array(vec![test1.clone(), test1.clone()].into());

    println!("{}", test1.test_value());
    println!("{}", test2.test_value());
}