#![feature(return_position_impl_trait_in_trait)]

pub struct Record<'a> {
    pub args: &'a [(usize, &'a str)],
}

mod a {
    trait Fooable<'a, 'b> {
        fn foo(&self) -> impl core::fmt::Debug;
    }

    impl<'a, 'b> Fooable<'a, 'b> for super::Record<'b> {
        fn foo(&self) -> impl core::fmt::Debug {
            println!("Foo called");
            42
        }
    }

    extern "Rust" {
        fn foo<'a, 'b>(record: &'a super::Record<'b>);
    }

    trait Barable<'a, 'b> {
        fn bar(&self) -> impl core::fmt::Debug;
    }

    impl<'a, 'b> Barable<'a, 'b> for super::Record<'b> {
        fn bar(&self) -> impl core::fmt::Debug {
            println!("Bar called");
            "Hello"
        }
    }

    extern "Rust" {
        fn bar<'a, 'b>(record: &'a super::Record<'b>);
    }
}

mod b {
    trait Fooable<'a, 'b> {
        fn foo(&self) -> impl core::fmt::Debug;
    }

    impl<'a, 'b> Fooable<'a, 'b> for super::Record<'b> {
        fn foo(&self) -> impl core::fmt::Debug {
            println!("Foo called");
            42
        }
    }

    extern "Rust" {
        fn foo<'a, 'b>(record: &'a super::Record<'b>);
    }

    trait Barable<'a, 'b> {
        fn bar(&self) -> impl core::fmt::Debug;
    }

    impl<'a, 'b> Barable<'a, 'b> for super::Record<'b> {
        fn bar(&self) -> impl core::fmt::Debug {
            println!("Bar called");
            "Hello"
        }
    }

    extern "Rust" {
        fn bar<'a, 'b>(record: &'a super::Record<'b>);
    }
}

fn main() {}