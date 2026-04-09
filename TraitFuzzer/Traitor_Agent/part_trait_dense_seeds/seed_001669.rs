trait NullCheck {
    fn is_null(&self) -> bool;
}

impl NullCheck for *const i32 {
    fn is_null(&self) -> bool {
        *self == 0 as *const i32
    }
}

pub fn main() {
    let i: *const i32 = 0 as _;
    assert!((&i as &dyn NullCheck).is_null());
}