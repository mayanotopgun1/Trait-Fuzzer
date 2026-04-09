trait Foo {
    type Item;
}

trait Bar
where
    Self: Foo,
{
}

trait BarExt: Bar
where
    Self: Foo,
{
    fn foo_send(&self)
    where
        Self::Item: Send,
    {
    }
}

impl<T> BarExt for T where T: Bar {}

#[allow(dead_code)]
fn foo<M>(_m: M)
where
    M: Bar,
    M::Item: Send,
{
}

fn main() {}