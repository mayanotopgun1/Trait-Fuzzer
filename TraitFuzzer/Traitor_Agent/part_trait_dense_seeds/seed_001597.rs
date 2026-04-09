#![feature(macro_attr)]

macro_rules! nest {
    (attr() { struct $name:ident; }) => {
        println!("nest");
        #[nest(1)]
        struct $name;
    };
    (attr(1) { struct $name:ident; }) => {
        println!("nest(1)");
        #[nest(2)]
        struct $name;
    };
    (attr(2) { struct $name:ident; }) => {
        println!("nest(2)");
    };
}

trait Nestable {
    fn nest(&self);
}

impl Nestable for S {
    fn nest(&self) {
        println!("nest");
    }
}

struct S;

fn main() {
    let s = S;
    s.nest();
}