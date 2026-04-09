#![allow(unreachable_patterns)]
#![allow(dead_code)]

enum Empty {}
enum Test1 {
    A(u8),
    B(Empty),
}
enum Test2 {
    A(u8),
    B(Empty),
    C,
}

trait FromEmpty {
    fn from_empty(_: Empty) -> Self;
}

impl FromEmpty for Test1 {
    fn from_empty(x: Empty) -> Self {
        Test1::B(x)
    }
}

impl FromEmpty for Test2 {
    fn from_empty(x: Empty) -> Self {
        Test2::B(x)
    }
}

fn bar() -> Option<Empty> {
    None
}

fn main() {
    if let Some(x) = bar() {
        let _ = Test1::from_empty(x);
    }

    if let Some(x) = bar() {
        let _ = Test2::from_empty(x);
    }
}