#![feature(type_alias_impl_trait)]
#![recursion_limit = "256"]

#[derive(PartialOrd, PartialEq)]
struct FloatWrapper(f64);

trait Compare {
    type Out: core::fmt::Debug + core::ops::Not<Output = Self::Out>;
    fn compare(&self, other: &Self) -> Self::Out;
}

trait CompareExt: Compare {
    fn compare_not(&self, other: &Self) -> Self::Out {
        !self.compare(other)
    }
}

impl<T> CompareExt for T where T: Compare {}

impl Compare for FloatWrapper {
    type Out = bool;

    fn compare(&self, other: &Self) -> Self::Out {
        self.0 == other.0
    }
}

fn main() {
    let nan1 = FloatWrapper(0.0 / 0.0);
    let nan2 = FloatWrapper(0.0 / 0.0);

    assert!(!nan1.compare_not(&nan2));
}