#![allow(stable_features)]

#![feature(thread_local_try_with)]
#![feature(impl_trait_in_assoc_type)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

struct Foo {
    cnt: usize,
}

trait InitFoo {
    fn init() -> Self;
}

impl InitFoo for Foo {
    fn init() -> Foo {
        let cnt = CNT.fetch_add(1, Ordering::SeqCst);
        if cnt == 0 {
            FOO.with(|_| {});
        }
        Foo { cnt: cnt }
    }
}

thread_local!(static FOO: Foo = <Foo as InitFoo>::init());

static CNT: AtomicUsize = AtomicUsize::new(0);

trait DropFoo {
    fn drop(&mut self);
}

impl Drop for Foo {
    fn drop(&mut self) {
        if self.cnt == 1 {
            FOO.with(|foo| assert_eq!(foo.cnt, 0));
        } else {
            assert_eq!(self.cnt, 0);
            if FOO.try_with(|_| ()).is_ok() {
                panic!("should not be in valid state");
            }
        }
    }
}

fn main() {
    thread::spawn(|| {
        FOO.with(|_| {});
    })
    .join()
    .unwrap();
}