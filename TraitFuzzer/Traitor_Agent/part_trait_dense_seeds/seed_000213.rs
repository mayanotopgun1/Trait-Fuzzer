#![feature(type_alias_impl_trait)]

trait Collectible {
    type Item;
    fn collect_to_vec(self) -> Vec<Self::Item>;
}

impl<I: std::iter::IntoIterator> Collectible for I {
    type Item = <I as std::iter::IntoIterator>::Item;
    fn collect_to_vec(self) -> Vec<Self::Item> {
        self.into_iter().collect()
    }
}

type Foo = impl std::fmt::Debug + PartialEq<Vec<i32>>;

#[define_opaque(Foo)]
fn foo(b: bool) -> Foo {
    if b { vec![42_i32] } else { std::iter::empty().collect_to_vec() }
}

fn bar(b: bool) -> impl std::fmt::Debug + PartialEq<Vec<i32>> {
    if b { vec![42_i32] } else { std::iter::empty().collect_to_vec() }
}

fn main() {
    assert_eq!(foo(true), vec![42]);
    assert_eq!(foo(false), vec![]);
    assert_eq!(bar(true), vec![42]);
    assert_eq!(bar(false), vec![]);
}