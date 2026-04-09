trait Mirror {
    type Assoc;
}

impl<T> Mirror for T {
    type Assoc = T;
}

trait Test {}
impl Test for i64 {}
impl Test for u64 {}

trait MirrorTest<T: Mirror>: Test where <T as Mirror>::Assoc: Test {
    fn mirror_and_test(t: T, s: <T as Mirror>::Assoc);
}
impl<U> MirrorTest<U> for U where U: Mirror + Test, <U as Mirror>::Assoc: Test {
    fn mirror_and_test(t: U, s: <U as Mirror>::Assoc) {}
}

fn main() {
    let mut x = 0;
    i64::mirror_and_test(x, 1);
    x = 1i64;
}