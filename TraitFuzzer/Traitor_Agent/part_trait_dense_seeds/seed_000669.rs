trait Callable {
    fn call(&mut self) -> isize;
}

impl<F> Callable for F
where
    F: FnMut() -> isize,
{
    fn call(&mut self) -> isize {
        (self)()
    }
}

fn foo(f: &mut dyn Callable) -> isize {
    f.call()
}

fn main() {
    let z = foo(&mut || 22);
    assert_eq!(z, 22);
}