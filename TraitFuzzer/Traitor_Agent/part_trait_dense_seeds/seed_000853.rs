pub trait Foo {
    fn load_from() -> Box<Self>;
    fn load() -> Box<Self> {
        Foo::load_from()
    }
}

trait FooExt: Foo {
    fn double_load() -> (Box<Self>, Box<Self>) {
        let a = Foo::load();
        let b = Foo::load();
        (a, b)
    }
}

impl<T: Foo> FooExt for T {}

pub fn load<M: Foo>() -> Box<M> {
    Foo::load()
}

fn main() { }