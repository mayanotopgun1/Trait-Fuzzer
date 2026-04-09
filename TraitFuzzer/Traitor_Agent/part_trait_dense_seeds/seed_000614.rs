use std::cell::RefCell;

struct S {
    f: Box<dyn FnMut()>
}

trait Call {
    fn call(&mut self);
}

impl Call for S {
    fn call(&mut self) {
        (self.f)();
    }
}

fn test(s: &RefCell<S>) {
    let mut guard = s.borrow_mut();
    guard.call();
}

fn main() {
    let s = RefCell::new(S {
        f: Box::new(|| ())
    });
    test(&s);
}