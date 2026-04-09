#![allow(dead_code)]

mod u8 {
    pub const BITS: usize = 8;
}

const NUM: usize = <u8 as U8Bits>::BITS;

trait U8Bits {
    const BITS: usize;
}

impl U8Bits for u8 {
    const BITS: usize = 8;
}

struct MyStruct { nums: [usize; NUM] }

fn main() {
    let _s = MyStruct { nums: [0; NUM] };
}