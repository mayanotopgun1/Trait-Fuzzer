#![feature(impl_trait_in_assoc_type)]

pub trait Trait: Supertrait {}

trait Impossible {}
impl<F: ?Sized + Impossible> Trait for F {}

pub trait Supertrait {}

trait SupertraitExt: Supertrait { fn extend(&self) {} }

impl<T: ?Sized + Trait + Impossible> Supertrait for T {}

fn needs_supertrait<T: ?Sized + Supertrait>() {}
fn needs_trait<T: ?Sized + Trait>() {}

struct A;
impl Trait for A where A: Supertrait {}
impl Supertrait for A {}
impl SupertraitExt for A {}

fn main() {
    needs_supertrait::<A>();
    needs_trait::<A>();
}