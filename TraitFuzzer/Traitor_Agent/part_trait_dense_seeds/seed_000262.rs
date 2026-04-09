#![feature(default_field_values, const_trait_impl, return_position_impl_trait_in_trait)]

const trait WTrait<const X: usize> {
    fn new() -> Self;
}

#[derive(Debug)]
struct W<const X: usize>;

impl<const X: usize> const WTrait<X> for W<X> {
    fn new() -> Self { W }
}

trait ZTrait<const X: usize> {
    const ONE: W<X>;
    const TWO: W<X>;
    const TOO_GENERIC: usize;

    fn create_instance(&self) -> impl core::fmt::Debug;
}

#[derive(Debug)]
struct Z<const X: usize> {
    one: W<X>,
    two: W<X>,
    too_generic: usize,
}

impl<const X: usize> ZTrait<X> for Z<X> {
    const ONE: W<X> = W::<X>::new();
    const TWO: W<X> = W::new();
    const TOO_GENERIC: usize = X + 1;

    fn create_instance(&self) -> impl core::fmt::Debug {
        self
    }
}

fn use_generically<const X: usize>() {
    let binding = Z::<X>::default();
    let z = ZTrait::<X>::create_instance(&binding);
    println!("{:?}", z);
}

impl<const X: usize> Default for Z<X> {
    fn default() -> Self {
        Z {
            one: W::new(),
            two: W::new(),
            too_generic: 0,
        }
    }
}

fn main() {
    let binding = Z::<0>::default();
    let z = ZTrait::<0>::create_instance(&binding);
    println!("{:?}", z);
    use_generically::<0>();
}