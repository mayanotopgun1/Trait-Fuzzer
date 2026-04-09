#![allow(dead_code)]
#![allow(non_camel_case_types)]

trait CatLike {
    fn new(done: extern "C" fn(usize)) -> Self;
}

trait DropLike {
    fn drop(&mut self);
}

struct cat {
    done : extern "C" fn(usize),
    meows : usize,
}

impl Drop for cat {
    fn drop(&mut self) {
        (self.done)(self.meows);
    }
}

impl CatLike for cat {
    fn new(done: extern "C" fn(usize)) -> Self {
        cat {
            meows: 0,
            done
        }
    }
}

fn main() {}