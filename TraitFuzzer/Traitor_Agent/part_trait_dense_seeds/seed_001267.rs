#![feature(return_position_impl_trait_in_trait)]

trait Negate { fn negate(self) -> impl Negate; }
impl Negate for f64 { fn negate(self) -> impl Negate { -self } }

fn main() {
    let _ = (-0.0).negate().negate();
}