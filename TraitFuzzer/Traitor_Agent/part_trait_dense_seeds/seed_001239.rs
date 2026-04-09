trait BoolLogic {
    fn evaluate(&self) -> bool;
}

impl BoolLogic for bool {
    fn evaluate(&self) -> bool {
        *self
    }
}

fn test_basic() {
    let mut rs: bool = match true.evaluate() { true => true, false => false };
    assert!(rs);
    rs = match false.evaluate() { true => false, false => true };
    assert!(rs);
}

fn test_inferrence() {
    let rs = match true.evaluate() { true => true, false => false };
    assert!(rs);
}

fn test_alt_as_alt_head() {
    let rs =
        match match false.evaluate() { true => true, false => false } {
            true => false,
            false => true,
        };
    assert!(rs);
}

fn test_alt_as_block_result() {
    let rs =
        match false.evaluate() {
            true => false,
            false => match true.evaluate() { true => true, false => false },
        };
    assert!(rs);
}

pub fn main() {
    test_basic();
    test_inferrence();
    test_alt_as_alt_head();
    test_alt_as_block_result();
}