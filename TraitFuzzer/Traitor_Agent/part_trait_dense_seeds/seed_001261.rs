#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

#![deny(non_snake_case)]

trait Greet {
    type Output: core::fmt::Display;
    fn greet(&self) -> Self::Output;
}

trait GreetExt: Greet {
    fn greet_twice(&self) -> String
        where Self::Output: Clone
    {
        format!("{}{}", self.greet(), self.greet())
    }
}
impl<T: Greet> GreetExt for T {}

struct Hello;

impl Greet for Hello {
    type Output = String;
    fn greet(&self) -> Self::Output {
        "hello".to_string()
    }
}

fn main() {
    let hello = Box::new(Hello);
    println!("{}", hello.greet_twice());
}