#![feature(box_patterns)]

#[derive(Clone)]
enum Noun {
    Atom(isize),
    Cell(Box<Noun>, Box<Noun>),
}

trait Fas {
    fn fas(&self) -> Noun;
}

impl Fas for Noun {
    fn fas(&self) -> Noun {
        match self {
            Noun::Cell(box Noun::Atom(2), box Noun::Cell(ref a, _)) => (**a).clone(),
            _ => panic!("Invalid fas pattern"),
        }
    }
}

pub fn main() {
    let noun = Noun::Cell(
        Box::new(Noun::Atom(2)),
        Box::new(Noun::Cell(Box::new(Noun::Atom(2)), Box::new(Noun::Atom(3)))),
    );
    noun.fas();
}