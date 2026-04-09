#![feature(specialization)]
#![allow(non_shorthand_field_patterns)]

struct Foo {
    x: isize,
    y: isize,
}

trait Matchable<'a> {
    type Item;
    fn match_structure(&'a self);
}

default impl<'a, T: 'a> Matchable<'a> for T {
    default type Item = &'a Self;
    default fn match_structure(&'a self) {
        println!("generic match");
    }
}

impl<'a> Matchable<'a> for Foo {
    type Item = &'a Self;
    fn match_structure(&'a self) {
        match self {
            Foo { x, y } => println!("yes, {}, {}", x, y),
        }
    }
}

trait MatchExt<'a>: Matchable<'a> {
    fn additional_match(&'a self);
}

impl<'a, T: Matchable<'a>> MatchExt<'a> for T {
    default fn additional_match(&'a self) {
        println!("additional match for {:?}", self.match_structure());
    }
}

impl<'a> MatchExt<'a> for Foo {
    fn additional_match(&'a self) {
        println!("Foo-specific additional match");
    }
}

pub fn main() {
    let a = Foo { x: 1, y: 2 };
    a.match_structure();
    a.additional_match();

    match a {
        Foo { .. } => ()
    }
}