#![allow(dead_code)]
#![feature(impl_trait_in_assoc_type)]

trait Negate {
    type Output;
    fn negate(&self) -> Self::Output;
}

trait NegateMut: Negate<Output = isize> {
    fn negate_mut(&mut self) -> isize {
        self.negate()
    }
}

trait NegateImm: Negate<Output = isize> {
    fn negate_imm(&self) -> isize {
        self.negate()
    }
}

impl Negate for isize {
    type Output = isize;
    fn negate(&self) -> Self::Output {
        -*self
    }
}

impl NegateMut for isize {}
impl NegateImm for isize {}

pub fn main() {}