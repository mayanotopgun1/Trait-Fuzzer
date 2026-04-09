#![feature(generic_associated_types)]

trait Incr {
    type Out<'a> where Self: 'a;
    fn incr<'a>(&'a mut self, amount: u8) -> Self::Out<'a>;
}

trait Decr {
    type Out<'a> where Self: 'a;
    fn decr<'a>(&'a mut self, amount: u8) -> Self::Out<'a>;
}

impl Incr for u8 {
    type Out<'a> = &'a u8;
    fn incr<'a>(&'a mut self, amount: u8) -> &'a u8 {
        *self = self.wrapping_add(amount);
        self
    }
}

impl Decr for u8 {
    type Out<'a> = &'a u8;
    fn decr<'a>(&'a mut self, amount: u8) -> &'a u8 {
        *self = self.wrapping_sub(amount);
        self
    }
}

pub fn main() {
    let mut x: u8 = 19;
    let mut y: u8 = 35;

    x.incr(7);
    y.decr(9);

    assert_eq!(x, y);
}