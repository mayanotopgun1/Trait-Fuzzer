#[repr(packed)]
pub struct Packed(i32);

trait MatchTrait {
    fn f(&self);
}

impl MatchTrait for Packed {
    fn f(&self) {
        match self {
            Packed(4) => {},
            _ if true => {},
            _ => {}
        }
    }
}

fn main() {
    let x = Packed(0);
    x.f();
}