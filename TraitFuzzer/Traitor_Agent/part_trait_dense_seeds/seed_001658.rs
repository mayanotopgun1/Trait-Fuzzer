#![feature(return_type_notation)]

trait IntFactory {
    type Stream<'a>: Iterator<Item = i32> where Self: 'a;
    fn stream(&self) -> Self::Stream<'static>;
}

trait SendIntFactory: for<'a> IntFactory<Stream<'a>: Send> + Send {}

struct DefaultIntFactory;

impl IntFactory for DefaultIntFactory {
    type Stream<'a> = std::iter::Once<i32>;
    fn stream(&self) -> Self::Stream<'static> {
        std::iter::once(42)
    }
}

impl SendIntFactory for DefaultIntFactory {}

fn main() {}