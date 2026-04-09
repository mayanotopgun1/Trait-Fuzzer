#![allow(incomplete_features)]
#![feature(loop_match)]

trait HelperTrait {
    fn helper(&self) -> u8;
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

fn main() {
    assert_eq!(().helper(), 1);
}