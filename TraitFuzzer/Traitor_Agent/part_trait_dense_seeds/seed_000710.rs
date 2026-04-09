#![feature(return_position_impl_trait_in_trait)]

trait Mir {
    fn run(&mut self);
}

struct M;

impl Mir for M {
    fn run(&mut self) {
        let mut y = 0;
        while y < 1 {
            y += 1;
        }
    }
}

fn mir() -> impl Mir {
    M
}

pub fn main() {
    let mut m = mir();
    m.run();
}