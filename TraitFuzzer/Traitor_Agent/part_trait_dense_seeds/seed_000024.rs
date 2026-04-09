#![feature(impl_trait_in_bindings)]

#[derive(Debug)]
struct Matrix4<S>(#[allow(dead_code)] S);

trait POrd<S> {}

trait Translate<S>: POrd<S> {
    fn translate(self) -> impl core::fmt::Debug;
}

impl<S: POrd<S> + std::fmt::Debug> Translate<S> for S {
    fn translate(self) -> impl core::fmt::Debug {
        Matrix4(self)
    }
}

impl POrd<f32> for f32 {}
impl POrd<f64> for f64 {}

fn main() {
    let x = 1.0;
    let m: impl core::fmt::Debug = x.translate();
    println!("m: {:?}", m);
}