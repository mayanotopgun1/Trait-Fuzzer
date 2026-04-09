const FOO: f64 = 10.0;

trait RangeCheck {
    fn is_within_range(&self, lower: f64, upper: f64) -> bool;
}

impl RangeCheck for f64 {
    fn is_within_range(&self, lower: f64, upper: f64) -> bool {
        *self >= lower && *self <= upper
    }
}

pub fn main() {
    let value = 0.0;

    match value.is_within_range(0.0, FOO) {
        true => (),
        false => ()
    }
}