trait SubAssignFromRef<T> {
    fn sub_assign_from_ref(&mut self, rhs: &T);
}

impl SubAssignFromRef<u8> for u8 {
    fn sub_assign_from_ref(&mut self, rhs: &u8) {
        *self -= *rhs;
    }
}

fn make_subtractor() -> impl FnMut(&mut u8, &u8) {
    |a, b| a.sub_assign_from_ref(b)
}

fn main() {
    let mut a: u8 = 0;
    let mut subtractor = make_subtractor();
    subtractor(&mut a, &1);
}