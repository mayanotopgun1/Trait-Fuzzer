#![feature(specialization)]

trait FContainer {
    fn get_f(&self) -> &Box<[Option<Self>; 1]>
    where
        Self: Sized;
}

default impl<T> FContainer for T {
    fn get_f(&self) -> &Box<[Option<Self>; 1]> {
        panic!("Default implementation should not be called");
    }
}

struct T {
    f: Box<[Option<T>; 1]>
}

impl FContainer for T {
    fn get_f(&self) -> &Box<[Option<Self>; 1]> {
        &self.f
    }
}

fn main() {
    let x = T { f: Box::new([None]) };
    let _ = x.get_f();
}