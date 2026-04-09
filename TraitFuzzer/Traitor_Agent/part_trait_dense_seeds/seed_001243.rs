trait CallTrait {
    fn call(&self, arg: &'static ());
}

impl<T> CallTrait for T
where
    T: Fn(&'static ()),
{
    fn call(&self, arg: &'static ()) {
        self(arg);
    }
}

fn foo(_: &()) {}

static X: fn(&'static ()) = foo;

fn main() {
    let _ = X.call(&());
}