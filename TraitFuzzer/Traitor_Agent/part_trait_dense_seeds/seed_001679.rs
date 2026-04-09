trait Incr { fn incr(&mut self, amount: u8); }
trait Decr { fn decr(&mut self, amount: u8); }

impl Incr for u8 {
    fn incr(&mut self, amount: u8) {
        *self = self.wrapping_add(amount);
    }
}

impl Decr for u8 {
    fn decr(&mut self, amount: u8) {
        *self = self.wrapping_sub(amount);
    }
}

pub fn main() {
    let mut x: u8 = 19;
    let mut y: u8 = 35;

    x.incr(7);
    y.decr(9);

    assert_eq!(x, y);
}