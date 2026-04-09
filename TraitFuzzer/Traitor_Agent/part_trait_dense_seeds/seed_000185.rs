#![expect(incomplete_features)]
#![feature(explicit_tail_calls)]

trait TailCall<'a> {
    fn r#become(&self) -> &'a [u8];
}

impl<'a> TailCall<'a> for () {
    fn r#become(&self) -> &'a [u8] {
        _g()
    }
}

fn _g() -> &'static [u8] {
    &[0, 1, 2, 3]
}

fn main() {}