trait ExtractPat { type Out; fn extract(&self, e: i32) -> Self::Out; }
trait DoubleExtract: ExtractPat where Self::Out: core::ops::Add<Output = Self::Out> + Copy {
    fn double_extract(&self, e: i32) -> Self::Out {
        let x = self.extract(e);
        x + x
    }
}
impl<T> DoubleExtract for T where T: ExtractPat, T::Out: core::ops::Add<Output = T::Out> + Copy {}

impl ExtractPat for i32 {
    type Out = i32;
    fn extract(&self, e: i32) -> Self::Out { e }
}

fn main() {
    let g1 = 13;
    let _ = g1.double_extract(g1);
}