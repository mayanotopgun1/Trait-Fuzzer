trait Foo<T> {
    fn get(&self) -> T;
}

trait FooExt<T>: Foo<T> where T: std::ops::Add<Output = T> + Copy {
    fn get_twice(&self) -> T {
        let v = self.get();
        v + v
    }
}

impl<S, T> FooExt<T> for S where S: Foo<T>, T: std::ops::Add<Output = T> + Copy {}

struct S {
    x: isize
}

impl Foo<isize> for S {
    fn get(&self) -> isize {
        self.x
    }
}

pub fn main() {
    let x = Box::new(S { x: 1 });
    let y: Box<dyn FooExt<isize>> = x as Box<dyn FooExt<isize>>;
    assert_eq!(y.get_twice(), 2);
}