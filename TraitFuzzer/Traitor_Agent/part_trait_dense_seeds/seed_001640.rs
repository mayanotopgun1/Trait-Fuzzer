#![feature(generic_associated_types)]

struct A<const N: usize>;

struct X;

trait Inner {
    type Assoc<'a, const N: usize>;
    fn inner<'a>() -> Self::Assoc<'a, 3>;
}

impl Inner for X {
    type Assoc<'a, const N: usize> = A<N>;
    fn inner<'a>() -> Self::Assoc<'a, 3> {
        outer::<3>()
    }
}

fn outer<const N: usize>() -> A<N> {
    A
}

fn main() {
    let i: A<3usize> = outer::<3usize>();
    let o: A<3usize> = X::inner();
}