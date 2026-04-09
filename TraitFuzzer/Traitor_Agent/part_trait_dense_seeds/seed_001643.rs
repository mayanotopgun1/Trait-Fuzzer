#![feature(const_trait_impl, const_cmp)]

#[derive(Eq, PartialEq)]
pub struct Y(u8);
pub const GREEN: Y = Y(4);

trait IsGreen {
    fn is_green(&self) -> bool;
}

impl IsGreen for Y {
    fn is_green(&self) -> bool {
        match self {
            &GREEN => true,
            _ => false,
        }
    }
}

struct CustomEq;

impl Eq for CustomEq {}
impl const PartialEq for CustomEq {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[derive(PartialEq, Eq)]
#[allow(unused)]
enum Foo {
    Bar,
    Baz,
    Qux(CustomEq),
}

const trait MatchFoo {
    fn match_bar_baz(&self) -> bool;
    fn match_empty(&self) -> bool;
}

impl const MatchFoo for Foo {
    fn match_bar_baz(&self) -> bool {
        matches!(self, Foo::Bar | Foo::Baz)
    }

    fn match_empty(&self) -> bool {
        false
    }
}

const BAR_BAZ: Foo = if 42 == 42 {
    Foo::Bar
} else {
    Foo::Qux(CustomEq)
};

const EMPTY: &[CustomEq] = &[];

const fn test() {
    const foo: Foo = Foo::Qux(CustomEq);

    if const { foo.match_bar_baz() } {
        panic!();
    }

    if let Foo::Qux(inner) = foo {
        if inner.eq(&CustomEq) {
            panic!();
        }
    }
}

impl const MatchFoo for &[CustomEq] {
    fn match_bar_baz(&self) -> bool {
        false
    }

    fn match_empty(&self) -> bool {
        self.is_empty()
    }
}

fn main() {}