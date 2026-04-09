#![allow(stable_features)]

#![feature(cfg_target_feature)]
#![feature(return_position_impl_trait_in_trait)]

trait ParseFloat {
    fn parse_f64(&self) -> Result<f64, std::num::ParseFloatError>;
}

impl ParseFloat for &str {
    fn parse_f64(&self) -> Result<f64, std::num::ParseFloatError> {
        self.parse::<f64>()
    }
}

trait FloatAssertions: ParseFloat {
    fn assert_positive_checker(&self) -> impl Fn() -> bool + '_ {
        let value = self;
        move || match value.parse_f64() {
            Ok(x) => x > 0.0,
            Err(_) => false,
        }
    }

    fn assert_identity_checker(&self) -> impl Fn() -> bool + '_ {
        let value = self;
        move || match value.parse_f64() {
            Ok(x) => (x + 0.0).abs_sub(3.1415).lt(&f64::EPSILON),
            Err(_) => false,
        }
    }

    fn assert_addition_checker(&mut self) -> impl FnMut() -> bool + '_ {
        let value = self;
        move || match value.parse_f64() {
            Ok(mut x) => {
                x += 5.0;
                (x.abs_sub(8.1415)).lt(&f64::EPSILON)
            }
            Err(_) => false,
        }
    }

    fn assert_positive(&self) {
        if !self.assert_positive_checker()() {
            panic!("Value is not positive");
        }
    }

    fn assert_identity(&self) {
        if !self.assert_identity_checker()() {
            panic!("Value does not match identity check");
        }
    }

    fn assert_addition(&mut self) {
        if !self.assert_addition_checker()() {
            panic!("Addition check failed");
        }
    }
}

impl<'a> FloatAssertions for &'a str {}

#[cfg(any(not(target_arch = "x86"), target_feature = "sse2"))]
fn main() {
    let s = "3.1415";
    s.assert_positive();
    s.assert_identity();
    let mut s_mut = "3.1415";
    s_mut.assert_addition();
}

#[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
fn main() {}