#![feature(return_position_impl_trait_in_trait)]

trait Cloner<'a, T: Clone + 'a> {
    fn clone_into_fn(&self, t: T) -> impl FnMut() -> T + 'a;
}

impl<'a, T: Clone + 'a> Cloner<'a, T> for () {
    fn clone_into_fn(&self, t: T) -> impl FnMut() -> T + 'a {
        move || t.clone()
    }
}

fn main() {
    let mut f = ().clone_into_fn(42_u32);
    assert_eq!(f(), 42);

    let mut f = ().clone_into_fn("forty-two");
    assert_eq!(f(), "forty-two");

    let x = 42_u32;
    let mut f = ().clone_into_fn(&x);
    assert_eq!(f(), &x);

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Foo(usize, &'static str);

    let x = Foo(42, "forty-two");
    let mut f = ().clone_into_fn(x);
    assert_eq!(f(), x);
}