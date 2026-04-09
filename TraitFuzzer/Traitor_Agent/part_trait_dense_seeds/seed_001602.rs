#![allow(unused_mut)]
#![allow(unused_variables)]

trait Decoder {
    fn decode(&self) -> String;
}

impl Decoder for () {
    fn decode(&self) -> String {
        'outer: loop {
            let mut ch_start: usize;
            break 'outer;
        }
        "".to_string()
    }
}

pub fn main() {
    println!("{}", (()).decode());
}