#![feature(cfg_overflow_checks)]

trait CompilesDifferently {
    fn compiles_differently(&self) -> bool;
}

#[cfg(overflow_checks)]
impl CompilesDifferently for () {
    fn compiles_differently(&self) -> bool {
        true
    }
}

#[cfg(not(overflow_checks))]
impl CompilesDifferently for () {
    fn compiles_differently(&self) -> bool {
        false
    }
}

fn main() {
    let x: Box<dyn CompilesDifferently> = Box::new(());
    assert!(!cfg!(overflow_checks));
    assert!(!x.compiles_differently());
}