#![feature(generic_associated_types)]

pub struct Struct {}

pub trait Trait<'a> {
    type Assoc;

    fn method() -> Self::Assoc;
}

trait ExtendedTrait<'a>: Trait<'a> {
    fn extended_method(&self) -> bool { true }
}

impl<'a, T: Trait<'a>> ExtendedTrait<'a> for T {}

impl<'a> Trait<'a> for Struct {
    type Assoc = ();

    fn method() -> Self::Assoc {}
}

pub fn function<F, T>(f: F)
where
    F: for<'b> FnOnce(<T as Trait<'b>>::Assoc),
    T: for<'c> ExtendedTrait<'c>,
{
    f(T::method());
}

fn main() {
    let s = Struct {};
    println!("{}", s.extended_method());

    function::<_, Struct>(|_| {});
}