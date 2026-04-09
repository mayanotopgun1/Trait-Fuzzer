#![feature(impl_trait_in_assoc_type)]

#[warn(dead_code)]
#[deny(warnings)]

trait MainTrait<'a> {
    type Out: core::fmt::Debug + Into<i32>;
    fn run(&self) -> Self::Out;
}

trait MainTraitExt<'a>: MainTrait<'a> {
    fn extended_run(&self) -> i32 {
        self.run().into()
    }
}

impl<'a, T> MainTraitExt<'a> for T where T: MainTrait<'a> {}

impl<'a> MainTrait<'a> for () {
    type Out = i32;
    fn run(&self) -> Self::Out {
        unimplemented!()
    }
}

fn main() {
    let _ = ().extended_run();
}