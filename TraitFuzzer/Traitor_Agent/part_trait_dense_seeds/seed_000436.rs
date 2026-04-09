#![feature(return_position_impl_trait_in_trait)]

trait Callable {
    extern "C" fn call() -> impl core::fmt::Debug;
}

impl Callable for () {
    extern "C" fn call() -> impl core::fmt::Debug {
        other()
    }
}

#[allow(unreachable_code)]
fn main() {
    panic!("stop");
    let _ = <() as Callable>::call();
}

extern "C" fn other() {}