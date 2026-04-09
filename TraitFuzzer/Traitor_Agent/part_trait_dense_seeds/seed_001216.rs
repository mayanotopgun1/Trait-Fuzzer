#![allow(dead_code)]
#![feature(never_type)]
#![feature(exhaustive_patterns)]

struct Foo {
    field1: !,
    field2: Option<&'static Bar>,
}

struct Bar {
    field1: &'static Foo
}

trait MatchOption<T> {
    fn match_option(self);
}

impl<T> MatchOption<T> for Option<T> {
    fn match_option(self) {
        match self {
            Some(_) => (),
            None => ()
        }
    }
}

fn test_a() {
    let x: Option<Foo> = None;
    x.match_option();
}

fn test_b() {
    let x: Option<Bar> = None;
    x.match_option();
}

fn main() {}