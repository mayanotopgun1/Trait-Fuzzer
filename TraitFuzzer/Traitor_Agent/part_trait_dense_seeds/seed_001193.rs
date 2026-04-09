#![allow(dead_code)]

trait LoopExt {
    fn nested_loop(&self) -> usize;
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