#![allow(incomplete_features)]
#![feature(loop_match)]

enum State {
    A,
    B,
}

trait ExitOnDropTrait {
    fn exit(&self);
}

struct ExitOnDrop;

impl Drop for ExitOnDrop {
    fn drop(&mut self) {
        std::process::exit(0);
    }
}

impl ExitOnDropTrait for ExitOnDrop {
    fn exit(&self) {}
}

trait DropBombTrait {
    fn bomb(&self);
}

struct DropBomb;

impl Drop for DropBomb {
    fn drop(&mut self) {
        panic!("this must unwind");
    }
}

impl DropBombTrait for DropBomb {
    fn bomb(&self) {}
}

fn main() {
    let mut state = State::A;
    #[loop_match]
    'a: loop {
        state = 'blk: {
            match state {
                State::A => {
                    let _exit = ExitOnDrop;
                    let _bomb = DropBomb;

                    #[const_continue]
                    break 'blk State::B;
                }
                State::B => break 'a,
            }
        };
    }

    unreachable!();
}