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
    where F: FnOnce(A, &u32)
{
}

fn foo() {
    let handler = ();
    handler.handle(|x: u32, y| {});
}

fn bar() {
    let handler = ();
    handler.handle(|x: &u32, y| {});
}

fn main() { }