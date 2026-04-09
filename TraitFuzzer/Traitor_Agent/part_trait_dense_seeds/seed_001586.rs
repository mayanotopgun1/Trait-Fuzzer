const FOO: isize = 10;

trait ZeroSizedTrait {
    fn as_zst(&self) -> &();
}

impl ZeroSizedTrait for isize {
    fn as_zst(&self) -> &() {
        unsafe { std::mem::transmute(*self) }
    }
}

fn main() {
    match FOO.as_zst() {
        ZST => 9,
    };
}