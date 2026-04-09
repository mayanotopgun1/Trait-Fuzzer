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
    F: for<'a> FnOnce(<T as Trait<'a>>::Assoc),
    T: for<'b> ExtendedTrait<'b>,
{
    f(T::method());
}

fn main() {
    let s = Struct {};
    println!("{}", s.extended_method());

    function::<_, Struct>(|_| {});
}