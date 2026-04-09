#![feature(specialization)]

trait Trait<T> {
    type Assoc;
}

impl<T> Trait<T> for Vec<T> {
    default type Assoc = ();
}

impl Trait<u8> for Vec<u8> {
    type Assoc = u8;
}

trait ExtendedTrait<T>: Trait<T> {
    fn extended_method(&self) -> bool {
        true
    }
}

impl<T> ExtendedTrait<T> for Vec<T> where T: Default {}

impl<T> Trait<T> for String {
    default type Assoc = ();
}

impl Trait<<Vec<u8> as Trait<u8>>::Assoc> for String {}

impl ExtendedTrait<<Vec<u8> as Trait<u8>>::Assoc> for String {}

fn main() {}