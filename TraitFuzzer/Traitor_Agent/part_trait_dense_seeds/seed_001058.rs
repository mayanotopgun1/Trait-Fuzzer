#![allow(static_mut_refs)]

struct Temporary;

static mut DROPPED: isize = 0;

trait Droppable {
    fn drop(&mut self);
}

impl Droppable for Temporary {
    fn drop(&mut self) {
        unsafe { DROPPED += 1; }
    }
}

trait DoStuff {
    fn do_stuff(&self) -> bool;
}

impl DoStuff for Temporary {
    fn do_stuff(&self) -> bool { true }
}

fn borrow() -> Box<Temporary> { Box::new(Temporary) }

pub fn main() {
    let mut i = 0;

    while borrow().do_stuff() {
        i += 1;
        unsafe { assert_eq!(DROPPED, i) }
        if i > 5 {
            break;
        }
    }

    if borrow().do_stuff() {
        unsafe { assert_eq!(DROPPED, i + 1) }
    }
}