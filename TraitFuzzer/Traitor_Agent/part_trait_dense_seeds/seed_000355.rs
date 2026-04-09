#![feature(trait_alias, const_trait_impl, generic_associated_types)]

mod foo {
    pub const trait Bar {
        fn bar(&self) {}
    }
    pub const trait Baz {
        fn baz(&self) {}
    }

    impl const Bar for () {}
    impl const Baz for () {}

    pub const trait Foo = [const] Bar + Baz;

    pub const trait Quux<'a>: Foo {
        type Assoc<'b> where Self: 'b;
        fn quux(&self, a: &'a u8) -> Self::Assoc<'a>;
    }

    impl<'a> const Quux<'a> for () {
        type Assoc<'b> = &'b ();
        fn quux(&self, _: &'a u8) -> Self::Assoc<'a> { &() }
    }

    pub const trait BarExt: Bar {
        fn bar_twice(&self) {}
    }

    impl<T: Bar> const BarExt for T {}

    pub const trait BazExt: Baz {
        fn baz_twice(&self) {}
    }

    impl<T: Baz> const BazExt for T {}
}

use foo::{BarExt, BazExt, Foo, Quux as _};

const _: () = {

    ().bar();
    ().bar_twice();

    ().baz();
    ().baz_twice();

    let a: &u8 = &0;
    let _ = <()>::quux(&(), a);
};

fn main() {}