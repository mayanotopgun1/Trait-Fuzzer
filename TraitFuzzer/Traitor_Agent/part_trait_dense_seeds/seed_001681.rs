#[cfg(unknown_key = "value")]

trait EmptyTrait {}

#[cfg(unknown_key = "value")]
impl EmptyTrait for () {}

pub fn f() {}

fn main() {}