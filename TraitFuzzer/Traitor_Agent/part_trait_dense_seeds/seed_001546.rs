trait MatchNegOne {
    fn match_neg_one(&self) -> Result<(), String>;
}

impl MatchNegOne for i32 {
    fn match_neg_one(&self) -> Result<(), String> {
        match self {
            -1 => Ok(()),
            _ => Err(String::from("wat")),
        }
    }
}

trait Subtract {
    type Output;
    fn subtract(self, other: Self) -> Self::Output;
}

impl Subtract for i32 {
    type Output = i32;
    fn subtract(self, other: Self) -> Self::Output {
        self - other
    }
}

pub fn main() {
    let _ = (-1).match_neg_one().unwrap();
    assert_eq!(1.subtract(1), 0);
}