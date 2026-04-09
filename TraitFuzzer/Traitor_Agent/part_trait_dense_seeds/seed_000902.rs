#![allow(dead_code)]

macro_rules! foo {
    ($x:tt) => (trait AliasTrait { type T; } impl AliasTrait for $x<i32> { type T = i32; })
}

foo!(Box);

fn main() {}