#![feature(specialization)]

#[allow(arithmetic_overflow)]
trait BitShift { fn bitshift(&self) -> u16; }
default impl<T> BitShift for T {
    fn bitshift(&self) -> u16 { 0 }
}
impl BitShift for [u8; 1] {
    fn bitshift(&self) -> u16 { (self[0] as u16) << 8 }
}

fn main() {
    let data = [42];
    let _result: u16 = data.bitshift();
}