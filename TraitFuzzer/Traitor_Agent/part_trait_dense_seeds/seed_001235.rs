use std::ops::Deref;

trait Foo<P>
where
    P: Deref,
    <P as Deref>::Target: Sized,
{
    fn foo(_value: <P as Deref>::Target) -> impl core::fmt::Debug;
}

impl<P> Foo<P> for ()
where
    P: Deref,
    <P as Deref>::Target: Sized,
{
    fn foo(_value: <P as Deref>::Target) -> impl core::fmt::Debug {
        42
    }
}

fn main() {
    let _ = <() as Foo<Box<u32>>>::foo(2);
}