#![allow(dead_code)]

use std::mem;

struct Cat {
    x: isize,
}

struct Kitty {
    x: isize,
}

impl Drop for Kitty {
    fn drop(&mut self) {}
}

trait SizeOf {
    fn size_of() -> usize;
}

impl SizeOf for Cat {
    fn size_of() -> usize {
        mem::size_of::<Self>()
    }
}

impl SizeOf for Kitty {
    fn size_of() -> usize {
        mem::size_of::<Self>()
    }
}

#[cfg(target_pointer_width = "64")]
pub fn main() {
    assert_eq!(Cat::size_of(), 8 as usize);
    assert_eq!(Kitty::size_of(), 8 as usize);
}

#[cfg(target_pointer_width = "32")]
pub fn main() {
    assert_eq!(Cat::size_of(), 4 as usize);
    assert_eq!(Kitty::size_of(), 4 as usize);
}