mod a {
    pub(crate) use crate::S;
    pub trait SExt { fn new() -> Self; }
}
mod b {
    pub struct S;
    impl super::a::SExt for S { fn new() -> Self { S } }
}
use self::a::{S, SExt};
use self::b::*;

fn main() {
    let _ = S::new();
}