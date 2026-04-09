#![feature(return_position_impl_trait_in_trait)]

struct Foo;

trait FooTrait {
    fn foo(&self) -> impl std::fmt::Debug;
}

impl FooTrait for () {
    fn foo(&self) -> impl std::fmt::Debug {
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