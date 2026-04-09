trait SubAssignFromRef<T> {
    fn sub_assign_from_ref(&mut self, rhs: &T);
}

impl SubAssignFromRef<u8> for u8 {
    fn sub_assign_from_ref(&mut self, rhs: &u8) {
        *self -= *rhs;
    }
}

fn main() {
    let mut a: u8 = 0;
    a.sub_assign_from_ref(&1);
}