trait Modulo { fn modulo(&self, other: &Self) -> Self; }
impl Modulo for f64 { fn modulo(&self, other: &Self) -> Self { *self % *other } }

pub fn f() -> f64 {
    std::hint::black_box(-1.0).modulo(&std::hint::black_box(-1.0))
}

pub fn g() -> f64 {
    -1.0.modulo(&-1.0)
}

pub fn main() {
    assert_eq!(f().signum(), g().signum());
}