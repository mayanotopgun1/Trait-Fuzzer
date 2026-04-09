#![allow(unreachable_patterns)]

#[derive(PartialEq, Eq)]
enum Cake {
    BlackForest,
    Marmor,
}
use Cake::*;

trait CakeUtils {
    fn is_marmor(&self) -> bool;
}

impl CakeUtils for Cake {
    fn is_marmor(&self) -> bool {
        *self == Marmor
    }
}

const BOO: (Cake, Cake) = (Marmor, BlackForest);
const FOO: Cake = BOO.1;

const fn foo() -> Cake {
    Marmor
}

const WORKS: Cake = Marmor;

const GOO: Cake = foo();

fn main() {
    match BlackForest {
        FOO => println!("hi"),
        GOO => println!("meh"),
        WORKS => println!("möp"),
        _ => println!("bye"),
    }
}