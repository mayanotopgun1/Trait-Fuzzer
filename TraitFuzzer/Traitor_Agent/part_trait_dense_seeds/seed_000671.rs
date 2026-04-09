#![feature(return_position_impl_trait_in_trait)]

const HI: &str = "hi";

trait AssertEqExt<T> {
    fn assert_eq_ext(&self, other: T) -> impl std::fmt::Debug;
}

impl<T1, T2> AssertEqExt<T2> for T1
where
    T1: PartialEq<T2> + std::fmt::Debug,
    T2: std::fmt::Debug,
{
    fn assert_eq_ext(&self, other: T2) -> impl std::fmt::Debug {
        assert_eq!(self, &other);
        self
    }
}

fn main() {
    let _ = HI.assert_eq_ext("hi");
}