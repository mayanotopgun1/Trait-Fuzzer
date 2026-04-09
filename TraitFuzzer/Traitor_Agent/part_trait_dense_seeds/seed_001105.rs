trait Hello<'a> {
    fn hello() -> impl Sized + 'a;
}

struct H;

impl<'a> Hello<'a> for H {
    fn hello() -> impl Sized + 'a { }
}

fn outlives<'a, T: 'a>(_: T) {}

fn test<'a>() {
    outlives::<'a, _>(H::hello());
}

fn main() {}