macro_rules! a {
    ( ) => {
        impl<'b, T: e> c for d where f: 'b {
            fn h() -> impl e {
                T::g()
            }
        }
    };
}

trait e {
    fn g();
}

struct d;

impl e for d {
    fn g() {}
}

fn main() {}