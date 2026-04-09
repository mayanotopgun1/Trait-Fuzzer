#![feature(impl_trait_in_assoc_type)]

pub trait Trait<T> {
    const S: &'static str;
}

trait DoubleTrait<T>: Trait<T> {
    const SS: &'static str = Self::S;
}

impl<T> Trait<()> for T
where
    T: for<'a> Trait<std::marker::PhantomData<&'a ()>>,
{
    const S: &'static str = T::S;
}

impl<T> DoubleTrait<()> for T
where
    T: for<'a> Trait<std::marker::PhantomData<&'a ()>> + Trait<()>,
{
}

fn main() {}