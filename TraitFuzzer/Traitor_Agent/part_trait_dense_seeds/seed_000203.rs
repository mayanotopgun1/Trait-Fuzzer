#![deny(rust_2021_incompatible_closure_captures)]
#![allow(unused_must_use)]

trait FilterPredicate {
    fn call(&mut self) -> bool;
}

impl<F: FnMut() -> bool> FilterPredicate for F {
    fn call(&mut self) -> bool {
        self()
    }
}

fn filter_try_fold(
    predicate: &mut impl FilterPredicate,
) -> impl FnMut() -> bool + '_ {
    move || predicate.call()
}

fn main() {
    let mut pred = || true;
    filter_try_fold(&mut pred);
}