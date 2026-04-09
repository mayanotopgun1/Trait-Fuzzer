#![allow(dead_code)]

trait SetMany {
    fn set_many(&mut self, xs: &[usize]);
}

struct Box {
    x: usize,
}

impl SetMany for Box {
    fn set_many(&mut self, xs: &[usize]) {
        for x in xs {
            self.x = *x;
        }
    }
}

pub fn main() {}