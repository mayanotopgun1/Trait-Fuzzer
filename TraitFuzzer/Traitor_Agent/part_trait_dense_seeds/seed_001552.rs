struct Foo;

trait FooTrait {
    fn foo(&self) -> Foo;
}

impl FooTrait for () {
    fn foo(&self) -> Foo {
        panic!()
    }
}

impl Drop for Foo {
    fn drop(&mut self) {}
}

fn main() {
    use std::thread;
    let handle = thread::spawn(|| {
        let _ = &[().foo()];
    });
    let _ = handle.join();
}