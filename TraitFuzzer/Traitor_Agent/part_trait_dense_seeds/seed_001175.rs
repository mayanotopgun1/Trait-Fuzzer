#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

struct S;

trait StaticAccess {
    fn as_static(&self) -> &'static Self;
}

impl StaticAccess for S {
    fn as_static(&self) -> &'static Self {
        panic!()
    }
}

type F = for<'cx> fn(&'cx S) -> &'cx S;
fn want_F(f: F) {}

trait AsStaticRef {
    fn as_static_ref(&self) -> &'static Self;
}

impl<T: StaticAccess> AsStaticRef for T {
    fn as_static_ref(&self) -> &'static Self {
        self.as_static()
    }
}

type G = for<'cx> fn(&'cx S) -> &'static S;
fn want_G(f: G) {}

trait CrossLifetimeAccess {
    fn cross_lifetime(&self) -> &'static Self;
}

impl<T: StaticAccess> CrossLifetimeAccess for T {
    fn cross_lifetime(&self) -> &'static Self {
        self.as_static()
    }
}

fn foo(x: &S) -> &'static S {
    panic!()
}

fn bar<'a, 'b>(x: &'a S) -> &'b S {
    panic!()
}

fn baz(x: &S) -> &S {
    panic!()
}

fn supply_F() {
    want_F(foo);

    want_F(bar);

    want_F(baz);
}

pub fn main() {}