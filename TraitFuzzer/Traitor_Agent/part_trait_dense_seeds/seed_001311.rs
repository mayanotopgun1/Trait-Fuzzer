#![feature(type_alias_impl_trait, coroutines, coroutine_trait, stmt_expr_attributes, impl_trait_in_assoc_type)]
#![allow(dropping_copy_types)]

use std::ops::Coroutine;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};

static A: AtomicUsize = AtomicUsize::new(0);

struct B;

trait DropExt {
    type CustomDropFuture: Coroutine<Yield = (), Return = ()> + 'static;
    fn custom_drop(&mut self) -> Self::CustomDropFuture;
}

impl Drop for B {
    fn drop(&mut self) {
        A.fetch_add(1, Ordering::SeqCst);
    }
}

impl DropExt for B {
    type CustomDropFuture = impl Coroutine<Yield = (), Return = ()> + 'static;
    fn custom_drop(&mut self) -> Self::CustomDropFuture {
        let this = unsafe { std::ptr::read(self as *mut _) };
        #[coroutine]
        move || {
            drop(this);
        }
    }
}

fn main() {
    t1();
    t2();
    t3();
}

fn t1() {
    let mut b = B;
    let mut foo = #[coroutine] || {
        yield;
        Pin::new(&mut b.custom_drop()).resume(());
    };

    let n = A.load(Ordering::SeqCst);
    drop(Pin::new(&mut foo).resume(()));
    assert_eq!(A.load(Ordering::SeqCst), n);
    drop(foo);
    assert_eq!(A.load(Ordering::SeqCst), n + 1);
}

fn t2() {
    let b = B;
    let mut foo = #[coroutine] || {
        yield b;
    };

    let n = A.load(Ordering::SeqCst);
    drop(Pin::new(&mut foo).resume(()));
    assert_eq!(A.load(Ordering::SeqCst), n + 1);
    drop(foo);
    assert_eq!(A.load(Ordering::SeqCst), n + 1);
}

fn t3() {
    let mut b = B;
    let foo = #[coroutine] || {
        yield;
        Pin::new(&mut b.custom_drop()).resume(());
    };

    let n = A.load(Ordering::SeqCst);
    assert_eq!(A.load(Ordering::SeqCst), n);
    drop(foo);
    assert_eq!(A.load(Ordering::SeqCst), n + 1);
}