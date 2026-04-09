#![feature(specialization)]

#[cfg(foo)]
#[macro_use]
mod foo {
    macro_rules! bar {
        () => { true }
    }

    pub trait BarTrait {
        fn is_bar_true(&self) -> bool;
    }

    default impl<T> BarTrait for T {
        fn is_bar_true(&self) -> bool {
            false
        }
    }

    impl BarTrait for () {
        fn is_bar_true(&self) -> bool {
            bar!()
        }
    }
}

#[cfg(not(foo))]
#[macro_use]
mod foo {
    macro_rules! bar {
        () => { false }
    }

    pub trait BarTrait {
        fn is_bar_true(&self) -> bool;
    }

    default impl<T> BarTrait for T {
        fn is_bar_true(&self) -> bool {
            false
        }
    }

    impl BarTrait for () {
        fn is_bar_true(&self) -> bool {
            bar!()
        }
    }
}

pub fn main() {
    use foo::BarTrait;
    assert!(().is_bar_true());
}