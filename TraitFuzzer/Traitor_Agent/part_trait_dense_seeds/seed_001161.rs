#![allow(dead_code)]

pub fn main() {
    enum State { BadChar, BadSyntax }

    trait StateCheck {
        fn check(&self) -> bool;
    }

    impl StateCheck for State {
        fn check(&self) -> bool {
            match self {
                State::BadChar => true,
                _ => false,
            }
        }
    }

    match State::BadChar {
        _ if State::BadChar.check() => State::BadChar,
        State::BadChar | State::BadSyntax => panic!(),
    };
}