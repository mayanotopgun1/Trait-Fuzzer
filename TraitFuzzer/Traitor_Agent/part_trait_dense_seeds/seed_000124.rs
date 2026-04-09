#![feature(impl_trait_in_bindings)]

trait Copyable { }

impl<T: Copy> Copyable for T { }

#[allow(dead_code)]
fn run() {
    let _foo: Box<dyn Copyable + '_> = Box::new(());
}

fn main() {}