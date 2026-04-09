#![deny(unused_parens)]
#![allow(unreachable_code, unused_variables, dead_code)]
#![feature(never_type)]

trait OuterLoopControl {
    fn control(&self) -> bool;
}

impl OuterLoopControl for () {
    fn control(&self) -> bool {
        true
    }
}

impl OuterLoopControl for ! {
    fn control(&self) -> bool {
        unreachable!()
    }
}

fn foo() {
    let _x = || 'outer: loop {
        let inner = 'inner: loop {
            let i = Default::default();

            if (break 'outer i).control() {
                loop {
                    break 'inner 5i8;
                }
            } else if true {
                break 'inner 6;
            }
            break 7;
        };
        break inner < 8;
    };
}

fn main() {}