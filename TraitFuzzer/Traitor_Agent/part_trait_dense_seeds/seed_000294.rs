#![allow(non_camel_case_types)]

enum noption<T> { some(T), }

trait OptionTrait<T> {
    fn some(value: T) -> Self;
}

impl<T> OptionTrait<T> for noption<T> {
    fn some(value: T) -> Self {
        noption::some(value)
    }
}

struct Pair { x: isize, y: isize }

pub fn main() {
    let nop: noption<isize> = OptionTrait::some(5);
    match nop {
        noption::some(n) => {
            println!("{}", n);
            assert_eq!(n, 5);
        }
    }
    let nop2: noption<Pair> = OptionTrait::some(Pair { x: 17, y: 42 });
    match nop2 {
        noption::some(t) => {
            println!("{}", t.x);
            println!("{}", t.y);
            assert_eq!(t.x, 17);
            assert_eq!(t.y, 42);
        }
    }
}