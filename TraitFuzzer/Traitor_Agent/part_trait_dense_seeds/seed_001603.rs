#![feature(impl_trait_in_assoc_type)]
#![allow(unused_mut)]
#![allow(unused_variables)]

trait Decoder {
    type Output;
    fn decode(&self) -> Self::Output;
}

impl Decoder for () {
    type Output = impl core::fmt::Debug;
    fn decode(&self) -> Self::Output {
        'outer: loop {
            let mut ch_start: usize;
            break 'outer;
        }
        "".to_string()
    }
}

pub fn main() {
    println!("{:?}", (()).decode());
}