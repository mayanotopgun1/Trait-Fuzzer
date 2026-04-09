#![warn(unused)]

#[warn(unused_variables)]
#[expect(unused_variables)]

trait Identity { fn identity(&self) -> i32; }
impl Identity for i32 { fn identity(&self) -> i32 { *self } }

fn main() {
    let x = 2;
    let _ = x.identity();
}