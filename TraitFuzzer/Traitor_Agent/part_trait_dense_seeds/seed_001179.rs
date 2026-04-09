use std::panic;

trait PanicHook {
    fn set(self);
}

impl<T> PanicHook for T
where
    T: Fn(&std::panic::PanicInfo<'_>) + Send + Sync + 'static,
{
    fn set(self) {
        panic::set_hook(Box::new(self));
    }
}

pub fn main() {
    let hook = |_: &std::panic::PanicInfo| {
        eprintln!("LTOed auxiliary crate panic hook");
    };

    hook.set();

    run_compiler();
}

fn run_compiler() {
    panic!("ICEing");
}