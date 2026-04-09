#![feature(specialization)]

pub trait Fooey: Sized {
    type Context<'c> where Self: 'c;

    fn create_context(&self) -> Option<Box<dyn for<'c> Fn(&mut Self::Context<'c>)>> {
        None
    }
}

default impl<T> Fooey for T {
    default type Context<'c> = () where T: 'c;
    default fn create_context(&self) -> Option<Box<dyn for<'c> Fn(&mut Self::Context<'c>)>> {
        None
    }
}

pub struct Handle<E: Fooey>(Option<Box<dyn for<'c> Fn(&mut E::Context<'c>)>>);

fn tuple<T>() -> (Option<T>,) { (Option::None,) }

pub struct FooImpl {}
impl Fooey for FooImpl {
    type Context<'c> = &'c ();
    fn create_context(&self) -> Option<Box<dyn for<'c> Fn(&mut Self::Context<'c>)>> {
        Some(Box::new(|_| {}))
    }
}

impl FooImpl {
    pub fn fail1() -> Handle<Self> {
        let (tx,) = tuple();
        Handle(tx)
    }
}

fn main() {}