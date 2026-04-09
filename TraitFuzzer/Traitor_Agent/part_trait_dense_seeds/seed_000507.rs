#![allow(dead_code)]

trait Negate {
    fn negate(&self) -> isize;
}

trait NegateMut: Negate {
    fn negate_mut(&mut self) -> isize {
        self.negate()
    }
}

trait NegateImm: Negate {
    fn negate_imm(&self) -> isize {
        self.negate()
    }
}

impl Negate for isize {
    fn negate(&self) -> isize {
        -*self
    }
}

impl NegateMut for isize {}
impl NegateImm for isize {}

pub fn main() {}