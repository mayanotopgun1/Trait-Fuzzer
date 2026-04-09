#![feature(generic_associated_types)]

use std::marker::PhantomData;

struct Foo;

struct Bar<A> {
    a: PhantomData<A>,
}

impl Fooifier for Foo {
    type Assoc = Foo;
}

trait Fooifier {
    type Assoc;
}

trait Barifier<H> {
    fn barify(&self);
}

impl<H> Barifier<H> for Bar<H> {
    fn barify(&self) {
        println!("All correct!");
    }
}

impl<A> Bar<A> {
    fn this_shouldnt_crash(&self)
    where
        Self: Barifier<<Foo as Fooifier>::Assoc>,
    {
        self.barify();
    }
}

fn main() {
    let bar = Bar::<Foo> { a: PhantomData };
    bar.this_shouldnt_crash();
}