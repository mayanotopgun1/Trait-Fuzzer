#![feature(return_type_notation)]

trait IntFactory {
    type Stream: Iterator<Item = i32>;
    fn stream(&self) -> Self::Stream;
}

trait SendIntFactory: IntFactory<Stream: Send> + Send {}

struct DefaultIntFactory;

impl IntFactory for DefaultIntFactory {
    type Stream = std::iter::Once<i32>;
    fn stream(&self) -> Self::Stream {
        std::iter::once(42)
    }
}

impl SendIntFactory for DefaultIntFactory {}

fn main() {}