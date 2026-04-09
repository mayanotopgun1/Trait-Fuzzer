#![allow(dead_code)]

trait Foo<A> {
    fn foo(&self, a: A);
}

impl<A,F:Fn(A)> Foo<A> for F {
    fn foo(&self, _: A) { }
}

trait BazExt<F,A>: Foo<(A,)> where A: 'static {}
impl<F,A: 'static> BazExt<F,A> for F where F: Foo<(A,)> {}

fn baz<A,F:for<'a> Foo<(&'a A,)>>(_: F) { }

fn components<T,A>(t: fn(&A))
    where fn(&A) : for<'a> Foo<(&'a A,)>,
{
    baz(t)
}

fn main() {
}