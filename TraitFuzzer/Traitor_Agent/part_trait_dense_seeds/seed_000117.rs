#[derive(PartialEq)]
struct NonMatchable;

impl Eq for NonMatchable {}

#[derive(PartialEq, Eq)]
enum Foo {
    A(NonMatchable),
    B(*const u8),
}

trait Matchable {
    fn matches(&self, other: &Self) -> bool;
}

impl Matchable for Foo {
    fn matches(&self, other: &Self) -> bool {
        self == other
    }
}

const CONST: Foo = Foo::B(std::ptr::null());

fn main() {
    match CONST {
        ref x if x.matches(&CONST) => 0,
        _ => 1,
    };
}