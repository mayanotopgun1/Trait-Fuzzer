pub trait FooTrait {
    #[inline(never)]
    fn foo(&self, bar: usize) -> usize;
}

trait FooClone: FooTrait {
    fn clone_foo(&self, bar: usize) -> usize {
        self.foo(bar)
    }
}

impl<T: FooTrait> FooClone for T {}

impl FooTrait for () {
    fn foo(&self, bar: usize) -> usize {
        std::convert::identity(bar)
    }
}

fn main() {
    let _ = <() as FooClone>::clone_foo(&(), 0);
}