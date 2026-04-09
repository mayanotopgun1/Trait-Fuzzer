#![feature(type_alias_impl_trait)]

pub trait Poison {
    type Alias;
    fn poison(&self) -> Self::Alias where Self: 'static;
}

impl Poison for () {
    type Alias = ();
    fn poison(&self) -> Self::Alias where Self: 'static {
        ()
    }
}

trait PoisonExt: Poison {}
impl<T> PoisonExt for T where T: Poison {}

pub fn poison1() -> impl Sized
where
    (): 'static,
{
    ().poison()
}

pub fn poison2() -> impl Sized
where
    (): 'static,
{
    define_by_query((poison2, ()));
}

pub fn poison3() -> impl Sized
where
    (): 'static,
{
    ().poison()
}

trait Query {}
impl<Out, F: Fn() -> Out> Query for (F, Out) {}

fn define_by_query(_: impl Query) {}

fn main() {}