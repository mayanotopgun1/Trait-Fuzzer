#![feature(specialization)]

pub trait Foo<T> {
    fn func1<U>(&self, t: U, w: T);

    fn func2<U>(&self, t: U, w: T) where Self: Bar<T> {
        self.func3(t, w);
    }
}

trait Bar<T>: Foo<T> {
    fn func3<U>(&self, t: U, w: T);
}

default impl<T, F: Foo<T>> Bar<T> for F {
    default fn func3<U>(&self, t: U, w: T) {
        self.func1(t, w);
    }
}

impl<T, F: Foo<T>> Bar<T> for F
where
    F: AnotherTrait,
{
    fn func3<U>(&self, t: U, w: T) {
        // Specialized implementation
        self.func1(t, w);
        println!("Specialized func3 called!");
    }
}

trait AnotherTrait {}

impl AnotherTrait for i32 {}

pub fn main() {}