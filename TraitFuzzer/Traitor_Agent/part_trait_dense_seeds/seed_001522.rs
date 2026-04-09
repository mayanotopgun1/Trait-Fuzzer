pub trait Foo {
    fn bar(&self) -> usize;
}

trait FooExt: Foo {
    fn bar_default(&self) -> usize {
        self.bar()
    }
}

impl<T> FooExt for T where T: Foo {}

impl Foo for () {
    fn bar(&self) -> usize { 3 }
}

fn main() {
    let result = ().bar_default();
    assert_eq!(result, 3);
}