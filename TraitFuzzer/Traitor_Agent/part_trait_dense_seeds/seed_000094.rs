trait Foo {
    type Bar<T>
    where
        dyn Send + 'static: Send;
}

impl Foo for () {
    type Bar<T> = i32;
}

trait FooView: Foo {
    fn bar_ref(&self) -> <Self as Foo>::Bar<()> {
        unimplemented!()
    }
}

impl<T: Foo> FooView for T {}

trait FooExt: Foo {
    fn bar_default(&self) -> <Self as Foo>::Bar<i32> {
        unimplemented!()
    }
}

impl<T: Foo> FooExt for T {}

fn main() {}