trait AssertUnchecked {
    unsafe fn assert_unchecked(&self, cond: bool);
}

impl AssertUnchecked for () {
    unsafe fn assert_unchecked(&self, cond: bool) {
        std::hint::assert_unchecked(cond);
    }
}

fn main() {
    unsafe {
        let _ = ().assert_unchecked(false);
    }
}