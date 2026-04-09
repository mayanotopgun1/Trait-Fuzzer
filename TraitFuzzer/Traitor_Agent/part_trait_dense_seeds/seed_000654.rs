trait Foo
where
    Self::Iterator: Iterator,
    <Self::Iterator as Iterator>::Item: Bar,
{
    type Iterator;

    fn iter(&self) -> Self::Iterator;
}

trait Bar {
    fn bar(&self);
}

trait FooExt: Foo {
    fn first_bar(&self) {
        self.iter().next().unwrap().bar();
    }
}

impl<T: Foo> FooExt for T {}

fn x<T: FooExt>(t: &T) {
    t.first_bar();
}

fn main() {}