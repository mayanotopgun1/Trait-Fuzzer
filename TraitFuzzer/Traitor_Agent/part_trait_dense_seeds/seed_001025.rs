pub enum E<'a> {
    Empty,
    Some(&'a E<'a>),
}

trait Fuzzable {
    fn fuzz(&self) -> u32;
}

impl<'a> Fuzzable for E<'a> {
    fn fuzz(&self) -> u32 {
        if let E::Some(E::Some(_)) = self { 1 } else { 2 }
    }
}

fn main() {
    assert_eq!(E::Empty.fuzz(), 2);
}