#![feature(const_trait_impl)]
#![allow(dead_code)]
#![allow(unused_unsafe)]

const trait NonZeroU32Trait {
    unsafe fn new_unchecked(value: u32) -> Self;
}

#[derive(PartialEq, Eq)]
pub struct NonZeroU32 {
    value: u32,
}

impl const NonZeroU32Trait for NonZeroU32 {
    unsafe fn new_unchecked(value: u32) -> Self {
        NonZeroU32 { value }
    }
}

trait ZeroCheck {
    fn is_zero(&self) -> bool;
}

impl ZeroCheck for NonZeroU32 {
    fn is_zero(&self) -> bool {
        self.value == 0
    }
}

pub const FOO_ATOM: NonZeroU32 = unsafe { NonZeroU32Trait::new_unchecked(7) };

fn main() {
    match None {
        Some(FOO_ATOM) => {}
        _ => {}
    }

    println!("Is zero: {}", FOO_ATOM.is_zero());
}