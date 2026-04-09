#![feature(lazy_type_alias)]

struct Wr<T>(T);
trait Foo {}
impl Foo for Wr<i32> {}

type Alias<T> = (T,)
where Wr<T>: Foo;

trait Bar {
    fn bar(&self);
}

impl<T> Bar for Wr<T>
where
    Alias<T>: Into<(T,)>,
    Wr<T>: Foo,
{
    fn bar(&self) {}
}

fn hello<T>() where Alias<T>: Into<(T,)>, Wr<T>: Foo {}

fn main() {
    let w = Wr(42);
    w.bar();
}