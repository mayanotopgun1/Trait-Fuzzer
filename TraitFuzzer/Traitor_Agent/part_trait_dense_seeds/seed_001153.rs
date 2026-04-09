#![crate_type = "lib"]
#![feature(const_trait_impl)]

enum Foo {
    Variant1(bool),
    Variant2(bool),
}

const trait Checkable {
    fn check(&self) -> bool;
}

impl const Checkable for Foo {
    fn check(&self) -> bool {
        match self {
            Foo::Variant1(x) | Foo::Variant2(x) => *x,
        }
    }
}

const _: () = {
    let mut n = 0;
    while n < 2 {
        if Foo::Variant1(true).check() {}
        n += 1;
    }
};