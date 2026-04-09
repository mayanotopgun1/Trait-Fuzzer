#![feature(generic_associated_types)]

trait SliceCheck {
    type Slice<'a> where Self: 'a;
    fn check_uwu(&self);
}

trait SliceExt: SliceCheck + AsRef<[u8]> {
    fn is_uwu(&self) -> bool {
        match &self.as_ref()[0..3] {
            b"uwu" => true,
            _ => false,
        }
    }
}

impl<T: ?Sized + AsRef<[u8]> + SliceCheck> SliceExt for T {}

impl<T: ?Sized + AsRef<[u8]>> SliceCheck for T {
    type Slice<'a> = &'a [u8] where T: 'a;
    fn check_uwu(&self) {
        self.is_uwu();
    }
}

fn test(s: &[u8]) {
    s.check_uwu();
}

fn main() {}