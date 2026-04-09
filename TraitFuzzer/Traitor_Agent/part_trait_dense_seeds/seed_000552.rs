#![recursion_limit="20"]

trait Y {
    type P;
}

impl<'a> Y for C<'a> {
    type P = Box<X<C<'a>>>;
}

struct C<'a>(&'a ());
struct X<T: Y>(T::P);

trait YCheck: Y where Self::P: Send {}
impl<T: Y> YCheck for T where T::P: Send {}

fn is_send<S: Send>() {}

fn main() {
    is_send::<X<C<'static>>>();
}