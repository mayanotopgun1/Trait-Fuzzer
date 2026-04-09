#![feature(default_field_values, const_trait_impl)]

const trait WTrait<const X: usize> {
    fn new() -> Self;
}

struct W<const X: usize>;

impl<const X: usize> const WTrait<X> for W<X> {
    fn new() -> Self { W }
}

trait ZTrait<const X: usize> {
    const ONE: W<X>;
    const TWO: W<X>;
    const TOO_GENERIC: usize;

    fn create_instance() -> Self;
}

struct Z<const X: usize> {
    one: W<X>,
    two: W<X>,
    too_generic: usize,
}

impl<const X: usize> ZTrait<X> for Z<X> {
    const ONE: W<X> = W::<X>::new();
    const TWO: W<X> = W::new();
    const TOO_GENERIC: usize = X + 1;

    fn create_instance() -> Self {
        Z {
            one: Self::ONE,
            two: Self::TWO,
            too_generic: Self::TOO_GENERIC,
        }
    }
}

fn use_generically<const X: usize>() {
    let _x: Z<X> = ZTrait::<X>::create_instance();
}

fn main() {
    let _x: Z<0> = ZTrait::<0>::create_instance();
    use_generically::<0>();
}