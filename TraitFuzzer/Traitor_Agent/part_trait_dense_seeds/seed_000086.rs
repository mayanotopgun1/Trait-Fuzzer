#![allow(unused_imports, overlapping_range_endpoints)]

use m::{START, END};

trait RangeCheck {
    fn in_range(&self, start: u32, end: u32) -> bool;
}

impl RangeCheck for u32 {
    fn in_range(&self, start: u32, end: u32) -> bool {
        *self >= start && *self <= end
    }
}

fn main() {
    match 42 {
        x if x.in_range(START, END) => {},
        x if x.in_range(0, END) => {},
        x if x.in_range(START, 59) => {},
        _ => {},
    }
}

mod m {
  pub const START: u32 = 4;
  pub const END:   u32 = 14;
}