#![feature(specialization)]

trait GoodTrait { fn good(&self, a: &isize); }
default impl<T> GoodTrait for T {
    fn good(&self, _a: &isize) {}
}
impl GoodTrait for () {
    fn good(&self, _a: &isize) {}
}

fn called<F>(f: F)
where
    F: FnOnce(&isize),
{
    f(&42);
}

pub fn main() {
    let _: () = ();
    called(|_| ().good(&42));
}