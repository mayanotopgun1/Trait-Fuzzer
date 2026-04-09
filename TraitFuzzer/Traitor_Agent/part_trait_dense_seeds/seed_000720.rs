#![allow(dead_code)]

trait MatchTrait {
    fn match_value(&self) -> Option<i32>;
}

impl MatchTrait for bool {
    fn match_value(&self) -> Option<i32> {
        match *self {
            true => Some(10),
            false => None,
        }
    }
}

fn f() {
    let x = true.match_value();
    if let Some(value) = x {
        println!("{}", value);
    } else {
        return;
    }
}

pub fn main() {}