pub trait Predicate<T> {
    fn apply(&self, value: T) -> bool;
}

impl<F: Fn(u32) -> bool> Predicate<u32> for F {
    fn apply(&self, value: u32) -> bool {
        self(value)
    }
}

fn main() {
    let predicate = |i: u32| i == 1;
    if Box::new(predicate).apply(5) {};
}