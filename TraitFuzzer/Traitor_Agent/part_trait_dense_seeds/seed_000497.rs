#![feature(cfg_overflow_checks)]

trait CompilesDifferently {
    fn compiles_differently() -> bool;
}

#[cfg(overflow_checks)]
impl CompilesDifferently for () {
    fn compiles_differently() -> bool {
        true
    }
}

#[cfg(not(overflow_checks))]
impl CompilesDifferently for () {
    fn compiles_differently() -> bool {
        false
    }
}

fn main() {
    assert!(!cfg!(overflow_checks));
    assert!(!<() as CompilesDifferently>::compiles_differently());
}