use std::ops::Deref;

trait Foo<P>
where
    P: Deref,
    <P as Deref>::Target: Sized,
{
    fn foo(_value: <P as Deref>::Target);
}

impl<P> Foo<P> for ()
where
    P: Deref,
    <P as Deref>::Target: Sized,
{
    fn foo(_value: <P as Deref>::Target) {}
}

fn main() {
    <() as Foo<Box<u32>>>::foo(2);
}