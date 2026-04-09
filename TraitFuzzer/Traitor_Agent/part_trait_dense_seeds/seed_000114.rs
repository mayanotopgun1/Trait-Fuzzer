use std::sync::mpsc::{Sender, channel};

trait Foo : Send + Sized + 'static {
    fn foo(self, tx: Sender<Self>) {
        tx.send(self).unwrap();
    }
}

trait FooExt: Foo {
    fn foo_twice(self, tx: Sender<Self>) where Self: Clone {
        tx.send(self.clone()).unwrap();
        tx.send(self).unwrap();
    }
}

impl<T: Foo + Clone> FooExt for T {}

impl <T: Send + 'static> Foo for T { }

pub fn main() {
    let (tx, rx) = channel();
    1193182.foo_twice(tx);
    assert_eq!(rx.recv().unwrap(), 1193182);
    assert_eq!(rx.recv().unwrap(), 1193182);
}