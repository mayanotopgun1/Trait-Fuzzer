#![deny(unused_qualifications)]

trait Defaultable {
    fn default_value() -> Self;
}

impl Defaultable for u64 {
    fn default_value() -> Self {
        ::std::default::Default::default()
    }
}

pub fn bar() -> u64 {
    u64::default_value()
}

fn main() {}