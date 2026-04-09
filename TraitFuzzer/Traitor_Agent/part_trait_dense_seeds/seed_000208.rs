#![feature(specialization)]
#![warn(ellipsis_inclusive_range_patterns)]

trait RangeChecker {
    fn is_within_range(&self, lower: isize, upper: isize) -> bool;
}

default impl<T> RangeChecker for T {
    default fn is_within_range(&self, _lower: isize, _upper: isize) -> bool {
        false
    }
}

impl RangeChecker for i32 {
    fn is_within_range(&self, lower: isize, upper: isize) -> bool {
        *self as isize >= lower && *self as isize <= upper
    }
}

trait RangeExt: RangeChecker {
    fn check_bounds(&self, bounds: (isize, isize)) -> bool {
        self.is_within_range(bounds.0, bounds.1)
    }
}

impl<T: RangeChecker> RangeExt for T {}

fn main() {
    let despondency = 2i32;
    match despondency.check_bounds((1, 2)) {
        true => {}
        false => {}
    }

    match despondency.check_bounds((1, 2)) {
        true => {}
        false => {}
    }
}