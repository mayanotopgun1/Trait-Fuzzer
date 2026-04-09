#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(impl_trait_in_bindings)]

trait Foo {
    type Output;

    fn foo() -> Self::Output;
}

impl<const N: usize> Foo for [u8; N] {
    type Output = [u8; N];

    fn foo() -> [u8; N] {
        [1u8; N]
    }
}

trait FooExt<const N: usize>: Foo<Output = [u8; N]> {
    fn foo_ref(&self) -> &[u8; N];
}

impl<const N: usize> FooExt<N> for [u8; N] {
    fn foo_ref(&self) -> &[u8; N] {
        self
    }
}

fn bug<const N: usize>()
where
    [u8; N]: FooExt<N>,
{
    let foo: impl AsRef<[u8]> = <[u8; N]>::foo();
    foo.as_ref();
}

fn main() {
    bug::<3>();
}