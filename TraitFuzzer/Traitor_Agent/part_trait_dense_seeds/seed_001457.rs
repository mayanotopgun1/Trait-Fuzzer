use std::borrow::Cow;

trait TestTrait<'a> {
    fn test_value(&self) -> Cow<'a, str>;
}

impl<'a> TestTrait<'a> for Test<'a> {
    fn test_value(&self) -> Cow<'a, str> {
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