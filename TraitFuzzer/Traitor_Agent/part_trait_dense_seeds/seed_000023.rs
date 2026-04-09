#[derive(Debug)]
struct Matrix4<S>(#[allow(dead_code)] S);

trait POrd<S> {}

trait Translate<S>: POrd<S> {
    fn translate(self) -> Matrix4<S>;
}

impl<S: POrd<S>> Translate<S> for S {
    fn translate(self) -> Matrix4<S> {
        Matrix4(self)
    }
}

impl POrd<f32> for f32 {}
impl POrd<f64> for f64 {}

fn main() {
    let x = 1.0;
    let m: Matrix4<f32> = x.translate();
    println!("m: {:?}", m);
}