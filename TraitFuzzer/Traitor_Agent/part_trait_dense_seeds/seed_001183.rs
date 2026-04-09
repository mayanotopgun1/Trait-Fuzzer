#![feature(generic_associated_types)]

pub fn main() {
    let _x: fn() = handle_debug_column;
}

fn handle_debug_column() {
    let sampler = sample_columns();

    let foo = || {
        sampler.get(17);
    };
    foo();
}

fn sample_columns() -> impl Sampler {
    ColumnGen {}
}

struct ColumnGen {}

trait Sampler {
    type Item<'a> where Self: 'a;
    fn get(&self, index: i32) -> Self::Item<'static>;
}

impl Sampler for ColumnGen {
    type Item<'a> = &'a str;
    fn get(&self, _index: i32) -> Self::Item<'static> {
        "Sample"
    }
}

trait SamplerExt: Sampler {
    fn get_twice(&self, index: i32) -> (Self::Item<'static>, Self::Item<'static>) where Self::Item<'static>: Clone;
}

impl<T: Sampler + 'static> SamplerExt for T where T::Item<'static>: Clone {
    fn get_twice(&self, index: i32) -> (Self::Item<'static>, Self::Item<'static>) {
        let item = self.get(index);
        (item.clone(), item)
    }
}