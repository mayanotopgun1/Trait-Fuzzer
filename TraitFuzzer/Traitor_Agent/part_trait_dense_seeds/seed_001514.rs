#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(loop_match)]

trait HelperTrait {
    fn helper(&self) -> u8;
}

default impl<T> HelperTrait for T {
    default fn helper(&self) -> u8 {
        let mut state = 0u8;
        #[loop_match]
        'a: loop {
            state = 'blk: {
                match state {
                    0 => break 'blk 1,
                    _ => break 'a state,
                }
            }
        }
    }
}

impl HelperTrait for () {
    fn helper(&self) -> u8 {
        let mut state = 0u8;
        #[loop_match]
        'a: loop {
            state = 'blk: {
                match state {
                    0 => break 'blk 1,
                    _ => break 'a state,
                }
            }
        }
    }
}

struct Custom;

impl HelperTrait for Custom {
    fn helper(&self) -> u8 {
        let mut state = 0u8;
        #[loop_match]
        'a: loop {
            state = 'blk: {
                match state {
                    0 => break 'blk 2,
                    _ => break 'a state,
                }
            }
        }
    }
}

fn main() {
    assert_eq!(().helper(), 1);
    assert_eq!(Custom.helper(), 2);
}