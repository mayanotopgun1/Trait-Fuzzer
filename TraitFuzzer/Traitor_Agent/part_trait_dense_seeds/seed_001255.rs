#![feature(return_position_impl_trait_in_trait)]

fn test_generic<T: Clone, F>(expected: T, eq: F) where F: FnOnce(T, T) -> bool {
    let actual: T = match true {
        true => expected.clone(),
        _ => panic!("wat")
    };
    assert!(eq(expected, actual));
}

trait TestGenericExt<F> where Self: Sized + Clone, F: FnOnce(Self, Self) -> bool {
    fn test_generic_ext(self, eq: F) -> bool;
}
impl<T, F> TestGenericExt<F> for T where Self: Clone, F: FnOnce(Self, Self) -> bool {
    fn test_generic_ext(self, eq: F) -> bool {
        let actual: Self = match true {
            true => self.clone(),
            _ => panic!("wat")
        };
        assert!(eq(self, actual));
        true
    }
}

fn test_vec() {
    fn compare_box(v1: Box<isize>, v2: Box<isize>) -> bool { return v1 == v2; }
    Box::new(1).test_generic_ext(compare_box);
}

pub fn main() { test_vec(); }