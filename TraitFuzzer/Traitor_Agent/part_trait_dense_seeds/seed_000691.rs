#![allow(dead_code)]

trait ModifyY {
    unsafe fn set_y(&self);
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