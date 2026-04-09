trait AssertUnchecked {
    unsafe fn assert_unchecked(&self, cond: bool);
}

impl AssertUnchecked for () {
    unsafe fn assert_unchecked(&self, cond: bool) {
        std::hint::assert_unchecked(cond);
    }
}

fn main() {
    let boxed_unit: Box<dyn AssertUnchecked> = Box::new(());
    unsafe {
        let _ = boxed_unit.assert_unchecked(false);
    }
}