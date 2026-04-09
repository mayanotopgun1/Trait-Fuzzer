#![feature(generic_associated_types)]

use foo::bar::{BazTrait, QuuxTrait};

mod foo {
    pub mod bar {
        pub trait BazTrait {
            type Item<'a> where Self: 'a;
            fn baz(&self) -> Self::Item<'_>;
        }

        pub trait QuuxTrait {
            type Item<'a> where Self: 'a;
            fn quux(&self) -> Self::Item<'_>;
        }

        pub trait BazQuuxExt: BazTrait + QuuxTrait {
            fn baz_and_quux(&self) -> (<Self as BazTrait>::Item<'_>, <Self as QuuxTrait>::Item<'_>) {
                (self.baz(), self.quux())
            }
        }

        impl<T> BazQuuxExt for T where T: BazTrait + QuuxTrait {}
    }
}

pub struct BazImpl;

impl foo::bar::BazTrait for BazImpl {
    type Item<'a> = &'a str;
    fn baz(&self) -> Self::Item<'_> {
        "baz"
    }
}

pub struct QuuxImpl;

impl foo::bar::QuuxTrait for QuuxImpl {
    type Item<'a> = &'a str;
    fn quux(&self) -> Self::Item<'_> {
        "quux"
    }
}

fn main() {
    let baz = BazImpl;
    let quux = QuuxImpl;

    let _result = (baz.baz(), quux.quux());
}