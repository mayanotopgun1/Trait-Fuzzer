trait Negate { fn negate(self) -> Self; }
impl Negate for f64 { fn negate(self) -> Self { -self } }
fn main() {
    let _ = (-0.0).negate().negate();
}