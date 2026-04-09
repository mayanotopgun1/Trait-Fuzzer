#![feature(return_position_impl_trait_in_trait)]
#![allow(non_camel_case_types)]

#[derive(Debug)]
enum noption<T> { some(T), }

trait OptionTrait<T>: core::fmt::Debug {
    fn some(value: T) -> Self;
}

impl<T: core::fmt::Debug> OptionTrait<T> for noption<T> {
    fn some(value: T) -> Self {
        noption::some(value)
    }
}

#[derive(Debug)]
struct Pair { x: isize, y: isize }

pub fn main() {
    let nop: noption<isize> = <noption<isize> as OptionTrait<isize>>::some(5);
    match nop {
        noption::some(n) => {
            println!("{}", n);
            assert_eq!(n, 5);
        }
    }
    let nop2: noption<Pair> = <noption<Pair> as OptionTrait<Pair>>::some(Pair { x: 17, y: 42 });
    match nop2 {
        noption::some(t) => {
            println!("{}", t.x);
            println!("{}", t.y);
            assert_eq!(t.x, 17);
            assert_eq!(t.y, 42);
        }
    }
}