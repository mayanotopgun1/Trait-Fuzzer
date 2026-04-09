#![feature(specialization)]
#![allow(stable_features)]
#![allow(unused_labels)]
#![allow(unreachable_code)]

macro_rules! x {
    ($a:lifetime) => {
        $a: loop {
            break $a;
            panic!("failed");
        }
    }
}

trait BreakExt {
    fn br(&self);
}

default impl<T> BreakExt for T {
    default fn br(&self) {}
}

impl<'a> BreakExt for &'a str {
    fn br(&self) {
        'c: loop {
            break 'c;
        }
    }
}

macro_rules! br2 {
    ($b:lifetime) => {
        $b: loop {
            break $b;
        }
    }
}

trait LoopExt {
    fn loop_br(&self);
}

default impl<T> LoopExt for T {
    default fn loop_br(&self) {}
}

impl<'a> LoopExt for &'a str {
    fn loop_br(&self) {
        'c: loop {
            self.br();
            break 'c;
        }
    }
}

fn main() {
    x!('a);
    "".loop_br();
    br2!('b);
    'b: loop {
        panic!("failed");
    }
}