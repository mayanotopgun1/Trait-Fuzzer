#[cfg(foo)]
#[macro_use]
mod foo {
    macro_rules! bar {
        () => { true }
    }

    pub trait BarTrait {
        fn is_bar_true(&self) -> bool;
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

    impl BarTrait for () {
        fn is_bar_true(&self) -> bool {
            bar!()
        }
    }
}

pub fn main() {
    use foo::BarTrait; // Import the trait to bring it into scope
    assert!(().is_bar_true());
}