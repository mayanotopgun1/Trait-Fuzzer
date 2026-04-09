#![feature(specialization)]

trait Main { fn main(&self); }

default impl<T> Main for T {
    fn main(&self) {}
}

struct M;
impl Main for M {
    fn main(&self) {
        println!("Specialized main for M");
    }
}

fn main() {
    let m = M;
    m.main();
}