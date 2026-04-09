#![deny(warnings)]

const X: u32 = 5;

trait Checkable<'a> {
    type Output;
    fn check(&'a self) -> Self::Output;
}

impl<'a> Checkable<'a> for &'a u32 {
    type Output = i32;
    fn check(&'a self) -> Self::Output {
        if **self > 10 {
            (**self - 10) as i32
        } else {
            (10 - **self) as i32
        }
    }
}

fn main() {
    let x = X;
    println!("{}", (&x).check());
}