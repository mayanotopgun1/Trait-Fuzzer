#![feature(impl_trait_in_assoc_type)]

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
    fn foo_send(&self) -> impl Send
    where
        Self::Item: Send,
    {
        unimplemented!()
    }
}

impl<T> BarExt for T where T: Bar {}

#[allow(dead_code)]
fn foo<M>(_m: M) -> impl Send
where
    M: Bar,
    M::Item: Send,
{
    unimplemented!()
}

fn main() {}