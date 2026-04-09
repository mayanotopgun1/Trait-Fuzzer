#![feature(specialization)]

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let x: i32 = parse()?;
    Ok(())
}

trait Parse {}

impl Parse for i32 {}

#[derive(Debug)]
struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError")
    }
}

impl Error for ParseError {}

trait ParseTwice: Parse where Self: core::ops::Add<Output = Self> + Copy {
    fn parse_twice(&self) -> Self {
        *self + *self
    }
}

default impl<T> ParseTwice for T where T: Parse + core::ops::Add<Output = T> + Copy {}

fn parse<T: Parse>() -> Result<T, ParseError> {
    todo!()
}