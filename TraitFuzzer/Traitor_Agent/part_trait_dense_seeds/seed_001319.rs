trait A {
    fn g(&self) -> isize { 10 }
}

impl A for isize {}

trait B: A {
    fn h(&self) -> isize where Self: Sized { self.g() * 2 }
}

impl<S> B for S where S: A {}

trait BExt: B {
    fn double_h(&self) -> isize where Self: Sized { self.h() * 2 }
}

impl<T> BExt for T where T: B {}

fn f<T:B>(i: T) {
    assert_eq!(i.double_h(), 40);
}

pub fn main () {
    f(0);
}