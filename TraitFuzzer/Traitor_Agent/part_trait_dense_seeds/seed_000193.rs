#![feature(negative_impls)]
#![allow(dead_code)]

struct TestType;

trait TestTrait {
    fn dummy(&self) {}
}

trait TestView: TestTrait {
    fn view_dummy(&self) { self.dummy() }
}

impl<T: TestTrait> TestView for T {}

impl !TestTrait for TestType {}

fn main() {}