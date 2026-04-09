#![allow(unused_variables)]
#![allow(unused_assignments)]
#[derive(Debug)]
#[allow(dead_code)]
enum Foo {
    Bar(u32, u32),
    Baz(&'static u32, &'static u32)
}

trait Access {
    fn f(&self) -> u32;
    fn g(&self) -> u32;
}

impl Access for Foo {
    fn f(&self) -> u32 {
        FNUM
    }

    fn g(&self) -> u32 {
        GNUM
    }
}

static NUM: u32 = 100;

fn main () {
    let mut b = Foo::Baz(&NUM, &NUM);
    b = Foo::Bar(b.f(), b.g());
}

static FNUM: u32 = 1;
static GNUM: u32 = 2;