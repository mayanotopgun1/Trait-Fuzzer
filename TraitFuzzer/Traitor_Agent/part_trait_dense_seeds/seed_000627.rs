#![feature(specialization)]
#![feature(rustc_attrs)]
#![rustc_no_implicit_bounds]

pub trait Trait: Super {
    fn example_method(&self) -> Self::Output where Self::Output: Sized;
}

pub trait Super {
    type Output: Default;
}

default impl<T> Trait for T where T: Super, <T as Super>::Output: Default {
    fn example_method(&self) -> Self::Output {

        let output = <T as Super>::Output::default();
        output
    }
}

trait TraitExt: Trait {
    fn default_output(&self) -> Self::Output {
        self.example_method()
    }
}

impl<T> TraitExt for T where T: Trait {}

fn bound<T: Trait>() {}

fn visit_simd_operator<V: Super>() {
    bound::<dyn Trait<Output = <V as Super>::Output>>();
}

fn main() {


}