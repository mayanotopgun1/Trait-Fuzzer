trait Callable {
    extern "C" fn call();
}

impl Callable for () {
    extern "C" fn call() {
        other()
    }
}

#[allow(unreachable_code)]
fn main() {
    panic!("stop");
    <() as Callable>::call();
}

extern "C" fn other() {}