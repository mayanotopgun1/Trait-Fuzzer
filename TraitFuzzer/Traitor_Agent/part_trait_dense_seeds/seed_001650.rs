#![allow(dead_code)]

pub trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}

impl<T: Sized> Borrow<T> for T {
    fn borrow(&self) -> &T { self }
}

trait Foo {
    fn foo(&self, other: &Self);
}

trait Bar<K, Q>: Borrow<Q>
where
    K: Borrow<Q>,
    Q: Foo,
{
    fn bar(&self, q: &Q) {
        q.foo(self.borrow())
    }
}

impl<K, T> Bar<K, T> for MyTree<K>
where
    K: Borrow<T>,
    T: Foo,
    MyTree<K>: Borrow<T>, // Add this line to satisfy the trait bound
{

}

struct MyTree<K>(K);

fn main() {}