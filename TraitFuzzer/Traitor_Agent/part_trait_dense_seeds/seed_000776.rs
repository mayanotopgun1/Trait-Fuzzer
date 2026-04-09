#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_shorthand_field_patterns)]

#![feature(box_patterns)]

struct Foo {a: isize, b: usize}

enum bar { u(Box<Foo>), w(isize), }

trait Unpack {
    fn unpack(self) -> (isize, usize);
}

impl Unpack for bar {
    fn unpack(self) -> (isize, usize) {
        match self {
            bar::u(box Foo{ a: a, b: b }) => (a, b),
            _ => (0, 0),
        }
    }
}

pub fn main() {
    let v = {
        let (a, b) = bar::u(Box::new(Foo{ a: 10, b: 40 })).unpack();
        a + (b as isize)
    };
    assert_eq!(v, 50);
}