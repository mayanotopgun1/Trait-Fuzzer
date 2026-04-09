#[deny(dead_code)]
pub enum Foo {
    Bar {
        baz: isize
    }
}

trait BazAccess {
    fn get_baz(&self) -> impl core::fmt::Display;
}

impl BazAccess for Foo {
    fn get_baz(&self) -> impl core::fmt::Display {
        match self {
            Foo::Bar { baz } => *baz,
        }
    }
}

fn main() {
    let foo = Foo::Bar { baz: 42 };
    println!("Baz value: {}", foo.get_baz());
}