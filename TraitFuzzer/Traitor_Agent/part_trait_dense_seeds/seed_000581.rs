#![feature(f16, f128)]

trait NanCheck {
    fn is_nan(&self) -> bool;
}

impl NanCheck for f16 {
    fn is_nan(&self) -> bool {
        *self == f16::NAN || *self != *self
    }
}

impl NanCheck for f32 {
    fn is_nan(&self) -> bool {
        *self == f32::NAN || *self != *self
    }
}

impl NanCheck for f64 {
    fn is_nan(&self) -> bool {
        *self == f64::NAN || *self != *self
    }
}

impl NanCheck for f128 {
    fn is_nan(&self) -> bool {
        *self == f128::NAN || *self != *self
    }
}

fn main() {
    let x = 5f16;
    let _ = !x.is_nan();

    let _ = x.is_nan();


    let x = 5f32;
    let _ = !x.is_nan();

    let _ = x.is_nan();


    let x = 5f64;
    let _ = !x.is_nan();

    let _ = x.is_nan();


    let x = 5f128;
    let _ = !x.is_nan();

    let _ = x.is_nan();


    let b = &2.3f32;
    if !b.is_nan() {}


    let b = &2.3f32;
    if !b.is_nan() {}


    let _ =
        b.is_nan();

    #[allow(unused_macros)]
    macro_rules! nan { () => { f32::NAN }; }
    macro_rules! number { () => { 5f32 }; }

    let _ = nan!().is_nan() || number!() == nan!();

    let _ = !nan!().is_nan() && number!() != nan!();
}