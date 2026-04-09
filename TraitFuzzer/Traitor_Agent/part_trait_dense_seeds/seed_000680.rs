#[allow(arithmetic_overflow)]
trait BitShift { fn bitshift(&self) -> u16; }
impl BitShift for [u8; 1] { fn bitshift(&self) -> u16 { (self[0] as u16) << 8 } }

fn main() {
    let data = [42];
    let _result: u16 = data.bitshift();
}