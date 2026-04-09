#![allow(unused_parens)]

trait LoopControl {
    fn forever(self) -> !;
}

impl LoopControl for () {
    fn forever(self) -> ! {
        loop {}
    }
}

pub fn main() {
    if (1 == 2) { ().forever(); }
}