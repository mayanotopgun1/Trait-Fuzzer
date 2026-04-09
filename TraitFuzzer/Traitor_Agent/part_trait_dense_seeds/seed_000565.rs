#![allow(dead_code)]
#![feature(dyn_trait)]

pub trait Insertable {
    type Values;

    fn values(self) -> Self::Values;
}

impl<T> Insertable for Option<T>
where
    T: Insertable,
    T::Values: Default,
{
    type Values = T::Values;

    fn values(self) -> Self::Values {
        self.map(Insertable::values).unwrap_or_default()
    }
}

impl<'a, T> Insertable for &'a Option<T>
where
    Option<&'a T>: Insertable,
{
    type Values = <Option<&'a T> as Insertable>::Values;

    fn values(self) -> Self::Values {
        self.as_ref().values()
    }
}

impl<'a, T> Insertable for &'a [T]
{
    type Values = Self;

    fn values(self) -> Self::Values {
        self
    }
}

trait Unimplemented { }

trait Dummy { }

struct Foo<T> { t: T }

impl<'a, U> Dummy for Foo<&'a U>
where &'a U: Insertable
{ }

impl<T> Dummy for T
where T: Unimplemented
{ }

fn main() {
}