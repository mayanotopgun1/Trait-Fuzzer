#![allow(warnings)]

struct Wrap<'p> { p: &'p mut i32 }

trait DropTrait {
    fn custom_drop(&mut self);
}

impl<'p> Drop for Wrap<'p> {
    fn drop(&mut self) {
        self.custom_drop();
    }
}

impl<'p> DropTrait for Wrap<'p> {
    fn custom_drop(&mut self) {
        *self.p += 1;
    }
}

trait DropExt: DropTrait {}
impl<T: DropTrait> DropExt for T {}

fn main() {
    let mut x = 0;
    let wrap = Wrap { p: &mut x };
    std::mem::drop(wrap);
    x = 1;
}