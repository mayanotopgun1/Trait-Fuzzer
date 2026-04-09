#![feature(generic_associated_types)]
#![allow(overlapping_range_endpoints)]

trait CharMatch {
    type MatchResult<'a> where Self: 'a;
    fn match_char(&self, c: char) -> Self::MatchResult<'static>;
}

impl CharMatch for char {
    type MatchResult<'a> = &'a str where Self: 'a;
    fn match_char(&self, c: char) -> Self::MatchResult<'static> {
        match c {
            'a'..='b' if false => "one",
            'a' => "two",
            'a'..='b' => "three",
            _ => panic!("what?"),
        }
    }
}

fn main() {
    let x = 'a';
    let y = x.match_char(x);
    assert_eq!(y, "two");
}