trait BitConversion {
    type Bits;
    fn from_bits(bits: Self::Bits) -> Self;
    fn to_bits(self) -> Self::Bits;
}

impl BitConversion for f64 {
    type Bits = u64;
    fn from_bits(bits: Self::Bits) -> Self {
        f64::from_bits(bits)
    }
    fn to_bits(self) -> Self::Bits {
        self.to_bits()
    }
}

impl BitConversion for f32 {
    type Bits = u32;
    fn from_bits(bits: Self::Bits) -> Self {
        f32::from_bits(bits)
    }
    fn to_bits(self) -> Self::Bits {
        self.to_bits()
    }
}

pub fn main() {
    let f: f32 = BitConversion::from_bits(0x19873cc2 as u32);
    assert_eq!(BitConversion::to_bits(f), 0);
}