#![feature(impl_trait_in_assoc_type)]
#![allow(unused_variables)]

trait Container<T: ?Sized> { fn inner(&self) -> &T; }
impl<T: ?Sized> Container<T> for Node<T> {
    fn inner(&self) -> &T { &self.0 }
}

struct Node<T: ?Sized>(#[allow(dead_code)] T);

fn main() {
    let x: Box<Node<[isize]>> = Box::new(Node([]));
    let _ = x.inner();
}