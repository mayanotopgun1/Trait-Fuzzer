#[cfg(unit)]
mod unit {
    trait Foo {
        fn foo(&self);
    }

    struct S;

    impl Foo for S {
        fn foo(&self) {
            let _self: &dyn Foo = self;
        }
    }
}

#[cfg(tuple)]
mod tuple {
    trait Foo {
        fn foo(&self);
    }

    struct S(());

    impl Foo for S {
        fn foo(&self) {
            let _self: &dyn Foo = self;
        }
    }
}

#[cfg(struct_)]
mod struct_ {
    trait Foo {
        fn foo(&self);
    }

    struct S {}

    impl Foo for S {
        fn foo(&self) {
            let _self: &dyn Foo = self;
        }
    }
}

fn main() {}