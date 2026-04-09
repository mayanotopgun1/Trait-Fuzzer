trait PromotionTest<const N: usize> {
    fn promote(&self) -> &'static usize;
}

impl PromotionTest<13> for () {
    fn promote(&self) -> &'static usize {
        &(3 + 13)
    }
}

fn promotion_test() -> impl PromotionTest<13> {
    ()
}

fn main() {
    assert_eq!(promotion_test().promote(), &16);
}