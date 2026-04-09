trait B {
    fn f(&self);
}

trait TB: B {
    fn g(&self) { }
}

impl<U: TB> B for U {
    fn f(&self) { }
}

struct A;

impl TB for A {
}

fn main() {
    let a = A;
    let br = &a as &dyn TB;
    br.f();
    br.g();
}