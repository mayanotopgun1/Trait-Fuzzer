#![feature(return_position_impl_trait_in_trait)]

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
    fn get_twice(&self, index: i32) -> (impl core::fmt::Debug + 'static, impl core::fmt::Debug + 'static);
}

impl<T: Sampler + 'static> SamplerExt for T where T::Item<'static>: Clone + core::fmt::Debug + 'static {
    fn get_twice(&self, index: i32) -> (impl core::fmt::Debug + 'static, impl core::fmt::Debug + 'static) {
        let item = self.get(index);
        (item.clone(), item)
    }
}