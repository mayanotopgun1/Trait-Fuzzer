trait ClosureLike { fn call_mut(&mut self); }

impl<T> ClosureLike for T where T: FnMut() {
    fn call_mut(&mut self) {
        (self)();
    }
}

pub fn repro() -> impl ClosureLike {
    if true { || () } else { || () }
}

fn main() {}