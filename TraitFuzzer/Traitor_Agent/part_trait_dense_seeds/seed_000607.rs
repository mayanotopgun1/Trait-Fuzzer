mod a {
    pub(crate) use crate::S;
    pub trait SExt { fn new() -> Self where Self: Sized; }
}
mod b {
    pub struct S;
    impl super::a::SExt for S { fn new() -> Self { S } }
}
use self::a::{S, SExt};
use self::b::*;

fn main() {
    let _: Box<dyn SExt> = Box::new(S::new());
}