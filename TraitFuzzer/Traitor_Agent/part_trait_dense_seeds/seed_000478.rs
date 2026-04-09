#![allow(dead_code)]

enum E {
    A, B
}

trait MatchTrait {
    fn match_value(&self) -> bool;
}

impl MatchTrait for &E {
    fn match_value(&self) -> bool {
        match self {
            &E::A => true,
            &E::B => false,
        }
    }
}

fn main() {
    let e = &&E::A;
    if e.match_value() {
        // Handle E::A
    } else {
        // Handle E::B
    }
}