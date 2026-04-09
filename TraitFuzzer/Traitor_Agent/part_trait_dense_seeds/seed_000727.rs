#![feature(impl_trait_in_assoc_type)]

trait T {
    fn test<const A: i32>(&self) -> impl std::fmt::Debug + PartialEq<i32> { A }
}

struct S();

impl T for S {}

trait TExt: T {
    fn test_ext<const A: i32>(&self) -> impl std::fmt::Debug + PartialEq<i32> where Self: Sized { self.test::<A>() }
}

impl<S> TExt for S where S: T {}

fn main() {
    let foo = S();
    assert_eq!(foo.test_ext::<8i32>(), 8);
    assert_eq!(foo.test_ext::<16i32>(), 16);
}