trait Callable {
    fn call(&self, i: isize, called: &mut bool);
}

impl Callable for fn(isize, &mut bool) {
    fn call(&self, i: isize, called: &mut bool) {
        self(i, called);
    }
}

fn f(i: isize, called: &mut bool) {
    assert_eq!(i, 10);
    *called = true;
}

fn g<F: Callable>(f: F, called: &mut bool) {
    f.call(10, called);
}

pub fn main() {
    let mut called = false;
    let h = f as for<'a> fn(isize, &'a mut bool);
    g(h, &mut called);
    assert_eq!(called, true);
}