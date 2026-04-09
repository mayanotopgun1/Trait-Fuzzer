use std::mem::Discriminant;

trait DiscriminantLike where Self: Sized {
    fn discriminant(&self) -> Discriminant<Self>;
}

impl<T> DiscriminantLike for T where T: Sized {
    fn discriminant(&self) -> Discriminant<Self> {
        std::mem::discriminant(self)
    }
}

fn foo<T: DiscriminantLike>(x: T) {
    x.discriminant();
}

fn main() {}