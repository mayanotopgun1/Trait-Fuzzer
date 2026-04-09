struct A<const N: usize>;

struct X;

trait Inner {
    fn inner<const N: usize>() -> A<N>;
}

impl Inner for X {
    fn inner<const N: usize>() -> A<N> {
        outer::<N>()
    }
}

fn outer<const N: usize>() -> A<N> {
    A
}

fn main() {
    let i: A<3usize> = outer::<3usize>();
    let o: A<3usize> = X::inner::<3usize>();
}