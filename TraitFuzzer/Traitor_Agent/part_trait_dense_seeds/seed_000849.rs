#![feature(trait_alias)]

trait DisplayAndDebug = std::fmt::Display + std::fmt::Debug;

trait Foo = DisplayAndDebug;
trait Bar = DisplayAndDebug;

fn main() {}