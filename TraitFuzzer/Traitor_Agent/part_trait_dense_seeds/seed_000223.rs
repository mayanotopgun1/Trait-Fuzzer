#![feature(rustc_attrs)]
#![allow(dead_code)]

mod variant_struct_region {
    trait FooTrait<'a> {
        type Item;
        fn get_x(&self) -> &Self::Item;
    }

    struct Foo<'a> {
        x: &'a i32,
    }

    impl<'a> FooTrait<'a> for Foo<'a> {
        type Item = i32;
        fn get_x(&self) -> &Self::Item {
            self.x
        }
    }

    trait BarTrait<'a, 'b> {
        type Inner;
        fn get_foo(&self) -> &Self::Inner;
    }

    struct Bar<'a, 'b> {
        f: &'a Foo<'b>,
    }

    impl<'a, 'b> BarTrait<'a, 'b> for Bar<'a, 'b> {
        type Inner = Foo<'b>;
        fn get_foo(&self) -> &Self::Inner {
            self.f
        }
    }
}

fn main() { }