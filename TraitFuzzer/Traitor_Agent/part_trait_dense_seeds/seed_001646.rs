#![allow(overlapping_range_endpoints)]

trait CharMatch {
    fn match_char(&self, c: char) -> &'static str;
}

impl CharMatch for char {
    fn match_char(&self, c: char) -> &'static str {
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