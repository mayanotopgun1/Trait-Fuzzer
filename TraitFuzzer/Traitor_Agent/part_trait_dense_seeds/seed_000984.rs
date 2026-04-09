#[allow(unexpected_cfgs)]
mod aa {
    trait BarTrait {
        fn bar();
    }

    impl BarTrait for () {
        #[cfg(true)] // Change cfg(false) to cfg(true) to ensure the function is implemented
        fn bar() {}
    }
}

fn main() {}