#![feature(generic_associated_types, specialization)]

trait GetRef<'a> {
    type Out;
    fn get(&self) -> &'a isize;
}

trait AddWithExt<'a, G: GetRef<'a>> where Self: GetRef<'a>, <Self as GetRef<'a>>::Out: Copy {
    fn add_twice(&self, g2: G) -> isize {
        *self.get() + *g2.get()
    }
}

impl<'a, T: GetRef<'a>> AddWithExt<'a, T> for T where <T as GetRef<'a>>::Out: Copy {}

#[derive(Copy, Clone)]
struct Box<'a> {
    t: &'a isize
}

impl<'a> GetRef<'a> for Box<'a> {
    type Out = isize;
    fn get(&self) -> &'a isize {
        self.t
    }
}

pub fn main() {
    let b1 = Box { t: &3 };
    assert_eq!(b1.add_twice(b1), 6);
}