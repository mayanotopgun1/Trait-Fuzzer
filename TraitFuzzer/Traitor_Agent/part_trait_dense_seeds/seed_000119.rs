#![allow(dead_code)]
#![feature(type_alias_impl_trait)]

enum E {
    Foo { f: isize },
    Bar,
}

trait MatchFoo {
    type Predicate;
    fn match_foo(&self, f: isize) -> Self::Predicate;
}

impl MatchFoo for E {
    type Predicate = bool;
    fn match_foo(&self, f: isize) -> Self::Predicate {
        matches!(self, E::Foo { f: value } if *value == f)
    }
}

trait ExtendedMatchFoo: MatchFoo<Predicate = bool> {
    fn is_zero(&self) -> bool {
        self.match_foo(0)
    }
}

impl<T: MatchFoo<Predicate = bool>> ExtendedMatchFoo for T {}

pub fn main() {
    let e = E::Foo { f: 0 };
    if e.match_foo(1) {
        panic!();
    } else if e.is_zero() {
    } else {
        panic!();
    }
}