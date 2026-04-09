#![feature(specialization)]
#![allow(unused_parens)]

trait LoopControl {
    fn forever(self) -> !;
}

default impl<T> LoopControl for T {
    fn forever(self) -> ! {
        loop {}
    }
}

impl LoopControl for () {
    fn forever(self) -> ! {
        loop {}
    }
}

pub fn main() {
    if (1 == 2) { ().forever(); }
}