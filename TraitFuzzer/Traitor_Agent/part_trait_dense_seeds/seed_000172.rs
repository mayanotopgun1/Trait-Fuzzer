#![deny(non_camel_case_types)]

trait FooTrait {
    fn is_bar(&self) -> bool;
}

impl FooTrait for Foo {
    fn is_bar(&self) -> bool {
        matches!(self, Foo::bar)
    }
}

pub enum Foo {
    #[allow(non_camel_case_types)]
    bar
}

fn main() {
    let foo = Foo::bar;
    println!("{}", foo.is_bar());
}