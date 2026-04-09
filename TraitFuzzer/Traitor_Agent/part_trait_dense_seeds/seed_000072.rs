#![deny(warnings)]

const X: u32 = 5;

trait Checkable {
    fn check(&self) -> i32;
}

impl Checkable for u32 {
    fn check(&self) -> i32 {
        if *self > 10 {
            (self - 10) as i32
        } else {
            (10 - self) as i32
        }
    }
}

fn main() {
    let x = X;
    println!("{}", x.check());
}