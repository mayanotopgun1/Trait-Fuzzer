#![feature(type_alias_impl_trait)]

trait CountMembers {
    type Counter;
    fn count(&self) -> Self::Counter;
}

impl CountMembers for [usize] {
    type Counter = usize;
    fn count(&self) -> Self::Counter {
        match *self {
            []         => 0,
            [_]        => 1,
            [_, ref xs @ ..] => 1 + xs.count()
        }
    }
}

fn main() {
    assert_eq!([1, 2, 3, 4].count(), 4);
}