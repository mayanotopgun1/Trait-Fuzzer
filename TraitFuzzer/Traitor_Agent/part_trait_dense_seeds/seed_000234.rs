pub struct Record<'a> {
    pub args: &'a [(usize, &'a str)],
}

mod a {
    trait Fooable<'a, 'b> {
        fn foo(&self);
    }

    impl<'a, 'b> Fooable<'a, 'b> for super::Record<'b> {
        fn foo(&self) {}
    }

    extern "Rust" {
        fn foo<'a, 'b>(record: &'a super::Record<'b>);
    }

    trait Barable<'a, 'b> {
        fn bar(&self);
    }

    impl<'a, 'b> Barable<'a, 'b> for super::Record<'b> {
        fn bar(&self) {}
    }

    extern "Rust" {
        fn bar<'a, 'b>(record: &'a super::Record<'b>);
    }
}

mod b {
    trait Fooable<'a, 'b> {
        fn foo(&self);
    }

    impl<'a, 'b> Fooable<'a, 'b> for super::Record<'b> {
        fn foo(&self) {}
    }

    extern "Rust" {
        fn foo<'a, 'b>(record: &'a super::Record<'b>);
    }

    trait Barable<'a, 'b> {
        fn bar(&self);
    }

    impl<'a, 'b> Barable<'a, 'b> for super::Record<'b> {
        fn bar(&self) {}
    }

    extern "Rust" {
        fn bar<'a, 'b>(record: &'a super::Record<'b>);
    }
}

fn main() {}