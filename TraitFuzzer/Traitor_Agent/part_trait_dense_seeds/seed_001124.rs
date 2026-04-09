#![feature(lazy_type_alias, return_position_impl_trait_in_trait)]

struct Wr<T>(T);
trait Foo {}
impl Foo for Wr<i32> {}

type Alias<T> = (T,)
where Wr<T>: Foo;

trait Bar {
    fn bar(&self) -> impl core::fmt::Debug;
}

impl<T> Bar for Wr<T>
where
    Alias<T>: Into<(T,)>,
    Wr<T>: Foo,
{
    fn bar(&self) -> impl core::fmt::Debug { () }
}

fn hello<T>() where Alias<T>: Into<(T,)>, Wr<T>: Foo {}

fn main() {
    let w = Wr(42);
    let _ = w.bar();
}