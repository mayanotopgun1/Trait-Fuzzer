#[allow(unused_macro_rules)]
macro_rules! a {
    ($i:literal) => { "right" };
    ($i:tt) => { "wrong" };
}

trait MacroTrait {
    fn b(&self, i: u8) -> &'static str;
}

impl MacroTrait for () {
    fn b(&self, _i: u8) -> &'static str {
        a!(0)
    }
}

fn main() {
    let unit = ();
    assert_eq!(unit.b(0), "right");
}