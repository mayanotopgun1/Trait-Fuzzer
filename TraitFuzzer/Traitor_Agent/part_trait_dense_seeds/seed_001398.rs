trait Foo: Send {
    fn fooify(&self) -> i32;
}

impl Foo for isize {
    fn fooify(&self) -> i32 {
        *self as i32
    }
}

pub fn main() {
    let x = 42_isize;
    println!("{}", x.fooify());
}