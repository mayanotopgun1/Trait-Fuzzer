#![allow(dead_code)]
#![feature(specialization)]

trait ModifyY {
    unsafe fn set_y(&self);
}

default impl<T> ModifyY for T {
    unsafe fn set_y(&self) {}
}

impl ModifyY for () {
    unsafe fn set_y(&self) {
        Y = 1;
    }
}

static mut Y: u32 = 0;

fn main() {
    unsafe {
        let _ = ().set_y();
    }
}