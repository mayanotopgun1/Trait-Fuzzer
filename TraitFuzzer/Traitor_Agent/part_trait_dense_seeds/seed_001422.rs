#![allow(non_camel_case_types)]

type compare<T> = extern "Rust" fn(T, T) -> bool;

trait Compare {
    type Item;
    fn compare(&self, other: Self::Item) -> bool;
}

impl<T: Clone> Compare for compare<T> {
    type Item = T;
    fn compare(&self, other: T) -> bool {
        self(other.clone(), other)
    }
}

fn test_generic<T: Clone>(expected: T, eq: impl Compare<Item = T>) {
    let actual: T = match true { true => expected.clone(), _ => panic!("wat") };
    assert!(eq.compare(actual));
}

trait Testable {
    fn run_test(self);
}

impl Testable for bool {
    fn run_test(self) {
        fn compare_bool(b1: bool, b2: bool) -> bool {
            return b1 == b2;
        }
        test_generic::<bool>(self, compare_bool as compare<bool>);
    }
}

#[derive(Clone)]
struct Pair {
    a: isize,
    b: isize,
}

impl Testable for Pair {
    fn run_test(self) {
        fn compare_rec(t1: Pair, t2: Pair) -> bool {
            t1.a == t2.a && t1.b == t2.b
        }
        test_generic::<Pair>(self, compare_rec as compare<Pair>);
    }
}

pub fn main() {
    true.run_test();
    Pair { a: 1, b: 2 }.run_test();
}