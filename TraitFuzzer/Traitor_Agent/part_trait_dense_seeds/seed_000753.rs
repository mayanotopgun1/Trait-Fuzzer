#![feature(return_position_impl_trait_in_trait)]

trait Duh {}

impl Duh for i32 {}

trait Trait {
    type Assoc: Duh;
}

impl<F: Duh> Trait for F {
    type Assoc = F;
}

fn foo() -> impl Trait<Assoc = impl Send> {
    42
}

trait TraitExt: Trait where Self::Assoc: Send { fn get_assoc(&self) -> Self::Assoc; }
impl<T: Trait + Send> TraitExt for T where <T as Trait>::Assoc: Send {
    fn get_assoc(&self) -> Self::Assoc {
        self.get_assoc()
    }
}

fn main() {
}