#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]

enum Foo {
    Bar(Option<i8>, (), (), Vec<i32>),
    Baz,
}

trait FooTrait<'a> {
    type Item;
    fn match_bar(&'a self) -> Option<Self::Item>;
}

impl<'a> FooTrait<'a> for Foo {
    type Item = (i8, &'a Vec<i32>);
    fn match_bar(&'a self) -> Option<Self::Item> {
        match self {
            Foo::Baz => None,
            Foo::Bar(None, ..) => None,
            Foo::Bar(Some(n), .., v) => Some((*n, &v)),
        }
    }
}

trait FooTraitExt<'a>: FooTrait<'a> {
    fn assert_match(&'a self) -> bool;
}

impl<'a> FooTraitExt<'a> for Foo {
    fn assert_match(&'a self) -> bool {
        match self.match_bar() {
            Some((n, v)) => v.len() == 2 && n == 1,
            None => false,
        }
    }
}

pub fn main() {
    let foo = Foo::Bar(Some(1), (), (), vec![2, 3]);

    if !foo.assert_match() {
        panic!();
    }
}