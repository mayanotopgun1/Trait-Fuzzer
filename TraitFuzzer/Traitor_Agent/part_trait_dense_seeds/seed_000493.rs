#![allow(incomplete_features)]
#![feature(raw_dylib_elf, dyn_trait)]

#[link(name = "hack", kind = "raw-dylib")]
unsafe extern "C" {}

trait LinkHack {
    unsafe fn hack(&self);
}

impl LinkHack for () {
    unsafe fn hack(&self) {}
}

fn main() {
    let trait_object: Box<dyn LinkHack> = Box::new(());
    unsafe {
        trait_object.hack();
    }
}