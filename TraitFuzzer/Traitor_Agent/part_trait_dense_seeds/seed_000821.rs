pub struct Bar<const F: bool>;

trait FooLike {
    fn foo() -> impl std::fmt::Debug;
}

trait BarLike {
    fn bar();
}

impl FooLike for Bar<true> {
    fn foo() -> impl std::fmt::Debug {
        "Bar<true>"
    }
}

impl<const F: bool> BarLike for Bar<F> {
    fn bar() {}
}

fn main() {
    let _ = <Bar<true> as FooLike>::foo();
}