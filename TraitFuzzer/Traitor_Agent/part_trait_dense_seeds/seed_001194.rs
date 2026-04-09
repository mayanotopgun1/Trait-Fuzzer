#![feature(specialization)]
#![allow(dead_code)]

trait LoopExt {
    fn nested_loop(&self) -> usize;
}

default impl<T> LoopExt for T {
    fn nested_loop(&self) -> usize {
        0
    }
}

impl LoopExt for () {
    fn nested_loop(&self) -> usize {
        loop { loop { break; } }
    }
}

fn f<T: LoopExt>(t: T) {
   let _x: usize = t.nested_loop();
}

pub fn main() {
}