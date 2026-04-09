const C1: i32 = 0x12345678;
const C2: isize = C1 as i16 as isize;

trait AsU64 {
    fn as_u64(&self) -> u64;
}

impl AsU64 for E {
    fn as_u64(&self) -> u64 {
        *self as u64
    }
}

#[derive(Copy, Clone)]
enum E {
    V = C2
}

fn main() {
    assert_eq!(C2 as u64, E::V.as_u64());
}