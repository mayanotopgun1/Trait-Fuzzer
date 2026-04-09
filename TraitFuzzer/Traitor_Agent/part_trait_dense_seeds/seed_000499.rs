pub trait Foo<T> {
    fn func1<U>(&self, t: U, w: T);

    fn func2<U>(&self, t: U, w: T) where Self: Bar<T> {
        self.func3(t, w);
    }
}

trait Bar<T>: Foo<T> {
    fn func3<U>(&self, t: U, w: T);
}

impl<T, F: Foo<T>> Bar<T> for F {
    fn func3<U>(&self, t: U, w: T) {
        self.func1(t, w);
    }
}

pub fn main() {}