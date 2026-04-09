const HI: &str = "hi";

trait AssertEqExt<T> { fn assert_eq_ext(&self, other: T); }
impl<T1, T2> AssertEqExt<T2> for T1 where T1: PartialEq<T2> + std::fmt::Debug, T2: std::fmt::Debug {
    fn assert_eq_ext(&self, other: T2) {
        assert_eq!(self, &other);
    }
}

fn main() {
    HI.assert_eq_ext("hi");
}