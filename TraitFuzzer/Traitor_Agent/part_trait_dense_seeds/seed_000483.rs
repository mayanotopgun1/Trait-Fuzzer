#![allow(stable_features)]

#![feature(cfg_target_feature)]

trait ParseFloat {
    fn parse_f64(&self) -> Result<f64, std::num::ParseFloatError>;
}

impl ParseFloat for &str {
    fn parse_f64(&self) -> Result<f64, std::num::ParseFloatError> {
        self.parse::<f64>()
    }
}

trait FloatAssertions: ParseFloat {
    fn assert_positive(&self) {
        if let Ok(x) = self.parse_f64() {
            assert_eq!(false, x <= 0.0);
        }
    }

    fn assert_identity(&self) {
        if let Ok(x) = self.parse_f64() {
            assert_eq!(3.1415, x + 0.0);
        }
    }

    fn assert_addition(&mut self) {
        if let Ok(mut x) = self.parse_f64() {
            assert_eq!(8.1415, { x += 5.0; x });
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