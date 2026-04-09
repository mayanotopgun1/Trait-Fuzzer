#![allow(dead_code)]

#![deny(non_snake_case)]

trait Greeter { fn greet(); }
impl Greeter for () { fn greet() { crate::hello(); } }

fn hello() {}

fn main() { <() as Greeter>::greet(); }