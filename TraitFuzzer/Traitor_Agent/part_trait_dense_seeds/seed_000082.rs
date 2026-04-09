trait Super<'a, const N: usize> {}
trait Trait: for<'a> Super<'a, { 1 + 1 }> {
    fn check(&self) -> bool;
}
impl<T> Trait for T where T: for<'a> Super<'a, { 1 + 1 }> {
    fn check(&self) -> bool {
        true
    }
}
fn main() {}