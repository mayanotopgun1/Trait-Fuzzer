#![feature(return_position_impl_trait_in_trait)]

trait Id<T> {
    fn id(x: T) -> T;
}

trait IdExt<T>: Id<T> where T: Copy + std::fmt::Debug {
    fn id_twice(x: T) -> T {
        Self::id(Self::id(x))
    }
}

impl<S, T> IdExt<T> for S where S: Id<T>, T: Copy + std::fmt::Debug {}

impl<T: std::fmt::Debug> Id<T> for () {
    fn id(x: T) -> T {
        x
    }
}

fn quux<T: Copy + std::fmt::Debug>(x: T) -> T {
    let f = <() as IdExt<T>>::id_twice;
    return f(x);
}

pub fn main() {
    assert_eq!(quux(10), 10);
}