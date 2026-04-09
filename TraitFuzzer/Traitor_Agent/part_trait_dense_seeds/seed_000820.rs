pub struct Bar<const F: bool>;

trait FooLike {
    fn foo();
}

trait BarLike {
    fn bar();
}

impl FooLike for Bar<true> {
    fn foo() {}
}

impl<const F: bool> BarLike for Bar<F> {
    fn bar() {}
}

fn main() {}