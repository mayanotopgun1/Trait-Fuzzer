#![allow(static_mut_refs)]

use std::thread;

static mut HIT: usize = 0;

thread_local!(static A: Foo = Foo);

struct Foo;

trait DropExt {
    fn custom_drop(&mut self);
}

impl Drop for Foo {
    fn drop(&mut self) {
        unsafe {
            HIT += 1;
        }
    }
}

impl DropExt for Foo {
    fn custom_drop(&mut self) {}
}

fn main() {
    unsafe {
        assert_eq!(HIT, 0);
        thread::spawn(|| {
            assert_eq!(HIT, 0);
            A.with(|_| ());
            assert_eq!(HIT, 0);
        }).join().unwrap();
        assert_eq!(HIT, 1);
    }
}