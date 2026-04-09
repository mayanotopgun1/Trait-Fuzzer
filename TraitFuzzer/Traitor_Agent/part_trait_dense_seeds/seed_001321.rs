#![allow(non_shorthand_field_patterns)]

use std::mem;

trait Accessor {
    fn x(&self) -> isize;
    fn y(&self) -> isize;
}

struct S {
    x: isize,
    y: isize,
}

impl Accessor for S {
    fn x(&self) -> isize { self.x }
    fn y(&self) -> isize { self.y }
}

type S2 = S;

trait S4Accessor<U> {
    fn x(&self) -> U;
    fn y(&self) -> char;
}

struct S3<U, V> {
    x: U,
    y: V,
}

impl<U: Clone> S4Accessor<U> for S3<U, char> {
    fn x(&self) -> U { self.x.clone() }
    fn y(&self) -> char { self.y }
}

type S4<U> = S3<U, char>;

fn main() {
    let s = S2 {
        x: 1,
        y: 2,
    };
    match s {
        S2 {
            x: x,
            y: y
        } => {
            assert_eq!(x, 1);
            assert_eq!(y, 2);
        }
    }

    let s = S4 {
        x: 4,
        y: 'a'
    };
    match s {
        S4::<u8> {
            x: x,
            y: y
        } => {
            assert_eq!(x as isize, 1);
            assert_eq!(y, 'a');
            assert_eq!(mem::size_of_val(&x), 1);
        }
    };

    let s = S4::<u16> {
        x: 5,
        y: 'b'
    };
    match s {
        S4 {
            x: x,
            y: y
        } => {
            assert_eq!(x as isize, 5);
            assert_eq!(y, 'b');
            assert_eq!(mem::size_of_val(&x), 2);
        }
    };
}