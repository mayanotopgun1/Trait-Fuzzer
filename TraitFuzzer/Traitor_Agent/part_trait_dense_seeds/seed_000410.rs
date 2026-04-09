#![feature(specialization)]

enum Void {}

#[repr(transparent)]
struct NoReturn<T>(T, Void);

#[allow(dead_code)]
struct Large(u64, u64, u64);

trait NeverTrait<T> {
    fn never(&mut self) -> NoReturn<T>;
}

default impl<S, T> NeverTrait<T> for S {
    default fn never(&mut self) -> NoReturn<T> {
        panic!("Default implementation should not be called");
    }
}

impl NeverTrait<Large> for bool {
    fn never(&mut self) -> NoReturn<Large> {
        *self = true;
        panic!("catch this")
    }
}

fn main() {
    let mut correct = false;
    let never: fn(&mut bool) -> NoReturn<Large> = |correct| NeverTrait::never(correct);

    let never: fn(&mut bool) -> Large = unsafe { std::mem::transmute(never) };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| never(&mut correct)));
    assert!(result.is_err(), "function should have panicked");
    assert!(correct, "function should have stored `true` into `correct`");
}