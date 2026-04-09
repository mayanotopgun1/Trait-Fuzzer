use std::mem::Discriminant;

trait DiscriminantLike where Self: Sized {
    fn discriminant(&self) -> Discriminant<Self>;
}

impl<T> DiscriminantLike for T where T: Sized {
    fn discriminant(&self) -> Discriminant<Self> {
        std::mem::discriminant(self)
    }
}

fn foo<T: DiscriminantLike>(x: T) -> impl Fn() -> Discriminant<T> {
    move || x.discriminant()
}

fn main() {
    let f = foo(42);
    println!("{:?}", f());
}