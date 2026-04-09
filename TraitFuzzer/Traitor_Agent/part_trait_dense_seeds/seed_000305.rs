#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

enum thing { a, b, c, }

trait Call: FnOnce(isize) {}

impl<F> Call for F where F: FnOnce(isize) {}

fn foo<C>(it: C) -> impl Call
where
    C: Call,
{
    move |i| it(i)
}

pub fn main() {
    let mut x = true;
    match thing::a {
        thing::a => { x = true; let _ = foo(|_i| { }); }
        thing::b => { x = false; }
        thing::c => { x = false; }
    }
}