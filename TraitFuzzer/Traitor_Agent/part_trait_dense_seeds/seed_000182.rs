#![feature(box_patterns)]

struct Root {
    boxed: Box<SetOfVariants>,
}

struct SetOfVariants {
    lhs: SomeVariant,
    rhs: SomeVariant,
}

enum SomeVariant {
    A(A),
    B(B),
}

struct A(String);
struct B(String);

trait Display {
    fn display(&self);
}

impl Display for A {
    fn display(&self) {
        println!("a = {}", self.0);
    }
}

impl Display for B {
    fn display(&self) {
        println!("b = {}", self.0);
    }
}

fn main() {
    let root = Root {
        boxed: Box::new(SetOfVariants {
            lhs: SomeVariant::A(A(String::from("This is A"))),
            rhs: SomeVariant::B(B(String::from("This is B"))),
        }),
    };
    if let box SetOfVariants { lhs: SomeVariant::A(a), rhs: SomeVariant::B(b) } = root.boxed {
        a.display();
        b.display();
    }
}