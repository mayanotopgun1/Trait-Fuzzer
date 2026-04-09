#[derive(Clone, Default)]
struct MaybeCopy<T>(T);

impl Copy for MaybeCopy<u8> {}

trait CopyCheck { fn check_copy(&self); }
impl<T: Copy> CopyCheck for T {
    fn check_copy(&self) {
        println!("{}", std::any::type_name::<Self>());
    }
}

fn main() {
    let x = MaybeCopy::default();
    x.check_copy();
    [MaybeCopy::default(); 13];
}