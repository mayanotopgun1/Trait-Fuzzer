#![feature(specialization)]

trait Parent1 {
    type P1;
    fn p1(&self) -> Self::P1;
}

trait Parent1Ext: Parent1 {
    fn d(&self) -> i32;
}

impl<T: ?Sized + Parent1> Parent1Ext for T {
    default fn d(&self) -> i32 {
        42
    }
}

trait Parent2 {
    type P2;
    fn p2(&self) -> Self::P2;
}

trait Child : Parent1 + Parent2 {
    type C;
    fn c(&self) -> Self::C;
}

struct Foo;

impl Parent1 for Foo {
    type P1 = u16;
    fn p1(&self) -> Self::P1 {
        println!("p1");
        1
    }
}

impl Parent1Ext for Foo {
    default fn d(&self) -> i32 {
        42
    }
}

impl Parent2 for Foo {
    type P2 = u32;
    fn p2(&self) -> Self::P2 {
        println!("p2");
        2
    }
}

impl Child for Foo {
    type C = u8;
    fn c(&self) -> Self::C {
        println!("c");
        0
    }
}

fn main() {

    let x = &Foo as &dyn Child<C=u8,P1=u16,P2=u32>;
    x.c();
    x.p1();
    x.p2();
    x.d();

    let y = &Foo as &dyn Parent1<P1=u16>;
    y.p1();
    y.d();
    let z = &Foo as &dyn Parent2<P2=u32>;
    z.p2();

    let x1 = x as &dyn Parent1<P1=u16>;
    x1.p1();
    x1.d();
    let x2 = x as &dyn Parent2<P2=u32>;
    x2.p2();
}