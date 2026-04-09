#![feature(type_alias_impl_trait)]

trait Boxable {
    type Boxed;
    fn box_new(self) -> Self::Boxed;
}

impl<T: std::fmt::Debug + 'static> Boxable for T {
    type Boxed = Box<Self>;
    fn box_new(self) -> Self::Boxed {
        Box::new(self)
    }
}

fn main() {
    let _a = 1.box_new();
}