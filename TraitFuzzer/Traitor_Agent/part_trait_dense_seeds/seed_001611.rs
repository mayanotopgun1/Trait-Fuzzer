#![feature(specialization)]

#[derive(PartialEq, Debug)]
struct Foo {
    x: isize,
    y: isize,
    z: isize,
}

trait PartialEqExt<T>: PartialEq<T> {
    fn eq_ext(&self, other: &T) -> bool;
    fn ne_ext(&self, other: &T) -> bool;
}

default impl<T: PartialEq<U>, U> PartialEqExt<U> for T {
    default fn eq_ext(&self, other: &U) -> bool {
        self == other
    }
    default fn ne_ext(&self, other: &U) -> bool {
        self != other
    }
}

impl PartialEqExt<Foo> for Foo {
    fn eq_ext(&self, other: &Foo) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne_ext(&self, other: &Foo) -> bool {
        !(self.eq_ext(other))
    }
}

pub fn main() {
    let a = Foo { x: 1, y: 2, z: 3 };
    let b = Foo { x: 1, y: 2, z: 3 };
    assert_eq!(a, b);
    assert!(!(a != b));
    assert!(a.eq_ext(&b));
    assert!(!a.ne_ext(&b));
}