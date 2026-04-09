#![feature(impl_trait_in_assoc_type)]

trait EmptyTrait {}

impl EmptyTrait for () {}

fn make_unit() -> impl EmptyTrait {
    ()
}

fn main() {
    #[cfg(crossbeam_loom)]
    let _ = make_unit();
}