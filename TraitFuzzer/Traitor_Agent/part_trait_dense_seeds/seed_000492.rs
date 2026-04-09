#![allow(incomplete_features)]
#![feature(raw_dylib_elf)]

#[link(name = "hack", kind = "raw-dylib")]
unsafe extern "C" {}

trait LinkHack {
    unsafe fn hack(&self);
}

impl LinkHack for () {
    unsafe fn hack(&self) {}
}

fn main() {
    unsafe {
        let _ = ().hack();
    }
}