#![feature(type_alias_impl_trait)]

trait Construct {
    type Out;
    fn new() -> Self::Out;
}

struct Large {
    a: isize,
    b: isize,
    c: isize,
    d: isize,
    e: isize,
    f: isize,
    g: isize,
    h: isize,
    i: isize,
    j: isize,
    k: isize,
    l: isize,
}

type HiddenLarge = Large;

impl Construct for Large {
    type Out = Self;
    fn new() -> Self::Out {
        Large {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 0,
            i: 0,
            j: 0,
            k: 0,
            l: 0,
        }
    }
}

trait ConstructExt: Construct {
    fn default_instance() -> Self::Out { Self::new() }
}

impl<T: Construct> ConstructExt for T {}

fn f() {
    let _foo: Large = Large::default_instance();
}

pub fn main() { f(); }