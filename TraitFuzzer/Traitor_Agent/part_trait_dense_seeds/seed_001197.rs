struct Foo;

trait Matchable {
    fn is_foo(&self) -> bool;
}

impl Matchable for Foo {
    fn is_foo(&self) -> bool {
        true
    }
}

pub fn main() {
    let x: Foo = Foo;
    if x.is_foo() {
        println!("hi");
    }
}