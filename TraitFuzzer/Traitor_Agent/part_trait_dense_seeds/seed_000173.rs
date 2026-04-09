#![deny(non_camel_case_types)]
#![feature(return_position_impl_trait_in_trait)]

trait FooTrait {
    fn is_bar(&self) -> impl Fn() -> bool;
}

impl FooTrait for Foo {
    fn is_bar(&self) -> impl Fn() -> bool {
        move || matches!(self, Foo::bar)
    }
}

pub enum Foo {
    #[allow(non_camel_case_types)]
    bar
}

fn main() {
    let foo = Foo::bar;
    println!("{}", foo.is_bar()());
}