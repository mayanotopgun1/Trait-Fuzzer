trait CallTrait {
    fn call(&self, arg: &'static ());
}

impl CallTrait for fn(&'static ()) {
    fn call(&self, arg: &'static ()) {
        self(arg);
    }
}

fn foo(_: &()) {}

static X: fn(&'static ()) = foo;

fn main() {
    let _ = X.call(&());
}