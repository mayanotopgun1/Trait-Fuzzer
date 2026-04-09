#![feature(generic_const_exprs, specialization)]
#![allow(incomplete_features)]

struct Foo<const N: u8>([u8; N as usize])
where
    [(); N as usize]:;

struct Bar<const N: u8>([u8; (N + 2) as usize]) where [(); (N + 2) as usize]:;


struct Evaluatable<const N: u16>;
trait FooTrait<const N: u8> {
    fn create_foo() -> Foo<N> where [(); N as usize]:;
}

default impl<T, const N: u8> FooTrait<N> for T where Evaluatable<{N as usize as u16 }>: {
    default fn create_foo() -> Foo<N> {
        Foo([0; N as usize])
    }
}

impl<const N: u8> FooTrait<N> for () where Evaluatable<{N as usize as u16 }>: {
    fn create_foo() -> Foo<N> {
        Foo([1; N as usize])
    }
}

trait BarTrait<const N: u8>: FooTrait<N> {
    fn create_bar() -> Bar<N> where [(); (N + 2) as usize]:;
}

impl<T, const N: u8> BarTrait<N> for T
where
    T: FooTrait<N>,
    [(); N as usize]:,
    [(); (N + 2) as usize]:,
{
    default fn create_bar() -> Bar<N> {
        let foo = Self::create_foo();
        let mut bar_data = [0; (N + 2) as usize];
        bar_data[..N as usize].copy_from_slice(&foo.0);
        Bar(bar_data)
    }
}

fn foo<const N: u8>() where Evaluatable<{N as usize as u16 }>: {
    let _ = <()>::create_foo();
}

fn bar<const N: u8>() where Evaluatable<{N as usize as u16 }>:, [(); (N + 2) as usize]: {
    let _ = <()>::create_bar();
}

fn main() {}