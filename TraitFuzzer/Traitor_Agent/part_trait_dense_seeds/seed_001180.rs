#![feature(return_position_impl_trait_in_trait)]

use std::panic;

trait PanicHook {
    fn set(self) -> impl core::fmt::Debug;
}

impl<T> PanicHook for T
where
    T: Fn(&std::panic::PanicInfo<'_>) + Send + Sync + 'static,
{
    fn set(self) -> impl core::fmt::Debug {
        panic::set_hook(Box::new(self));
        "Panic hook set"
    }
}

pub fn main() {
    let hook = |_: &std::panic::PanicInfo| {
        eprintln!("LTOed auxiliary crate panic hook");
    };

    let result = hook.set();
    println!("{:?}", result);

    run_compiler();
}

fn run_compiler() {
    panic!("ICEing");
}