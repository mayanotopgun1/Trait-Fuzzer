trait ClosureHandler<F, A> {
    fn handle(self, f: F);
}

impl<F, A> ClosureHandler<F, A> for ()
where
    F: FnOnce(A, &u32),
{
    fn handle(self, f: F) {
        with_closure(f)
    }
}

fn with_closure<F, A>(_: F)
where
    F: FnOnce(A, &u32),
{
}

fn foo() -> impl core::fmt::Debug {
    let handler = ();
    handler.handle(|x: u32, _: &u32| {});
    "foo"
}

fn bar() -> impl core::fmt::Debug {
    let handler = ();
    handler.handle(|x: &u32, _: &u32| {});
    "bar"
}

fn main() {
    println!("{:?}", foo());
    println!("{:?}", bar());
}