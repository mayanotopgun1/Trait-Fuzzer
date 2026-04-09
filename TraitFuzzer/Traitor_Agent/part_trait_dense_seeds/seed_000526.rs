#![feature(return_position_impl_trait_in_trait)]

trait Testable {
    fn test(&self) -> bool;
}

impl Testable for Option<usize> {
    fn test(&self) -> bool {
        match self {
            Some(0 | _) => true,
            _ => false,
        }
    }
}

fn main() {
    assert!(Some(42).test());
    assert!(!None::<usize>.test());
}