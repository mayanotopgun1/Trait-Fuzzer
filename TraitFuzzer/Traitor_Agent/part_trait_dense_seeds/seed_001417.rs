trait Value {
    fn value(&self) -> isize;
}

impl Value for Stuff {
    fn value(&self) -> isize {
        match self {
            Stuff::Bar => QUUX,
        }
    }
}

const QUUX: isize = 5;

enum Stuff {
    Bar = QUUX,
}

fn main() {
    let s = Stuff::Bar;
    assert_eq!(s.value(), QUUX);
}