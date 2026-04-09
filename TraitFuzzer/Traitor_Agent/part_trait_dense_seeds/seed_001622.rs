#![allow(overflowing_literals)]

trait EnumBool {
    fn to_bool(self) -> bool;
}

impl EnumBool for E64 {
    fn to_bool(self) -> bool {
        match self {
            E64::H64 => true,
            E64::L64 => false,
        }
    }
}

impl EnumBool for E32 {
    fn to_bool(self) -> bool {
        match self {
            E32::H32 => true,
            E32::L32 => false,
        }
    }
}

pub enum E64 {
    H64 = 0x7FFF_FFFF_FFFF_FFFF,
    L64 = 0x8000_0000_0000_0000
}
pub enum E32 {
    H32 = 0x7FFF_FFFF,
    L32 = 0x8000_0000
}

pub fn f(e64: E64, e32: E32) -> (bool, bool) {
    (e64.to_bool(), e32.to_bool())
}

pub fn main() { }