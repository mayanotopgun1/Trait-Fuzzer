#![feature(const_trait_impl, type_alias_impl_trait)]
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
    type Predicate: ?Sized;
    fn is_zero(&self, predicate: &Self::Predicate) -> bool;
}

impl ZeroCheck for NonZeroU32 {
    type Predicate = dyn Fn(&NonZeroU32) -> bool;
    fn is_zero(&self, predicate: &Self::Predicate) -> bool {
        predicate(self)
    }
}

pub const FOO_ATOM: NonZeroU32 = unsafe { NonZeroU32Trait::new_unchecked(7) };

fn main() {
    match None {
        Some(FOO_ATOM) => {}
        _ => {}
    }

    let is_zero_predicate = |x: &NonZeroU32| x.value == 0;
    println!("Is zero: {}", FOO_ATOM.is_zero(&is_zero_predicate));
}