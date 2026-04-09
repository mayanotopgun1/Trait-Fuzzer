use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

trait Callable {
    fn call(&self);
}

impl<F> Callable for F
where
    F: Fn(&()) + 'static,
{
    fn call(&self) {
        (self)(&());
    }
}

trait WrapperTrait<T>: Callable {
    fn inner_call(&self);
}

impl<T: Callable> WrapperTrait<T> for Wrapper<T> {
    fn inner_call(&self) {
        self.0.call();
    }
}

pub struct Wrapper<T>(T);

impl<T: Callable> Callable for Wrapper<T> {
    fn call(&self) {
        self.inner_call();
    }
}

pub fn foo() {
    let mut map: HashMap<(), Box<dyn Callable>> = HashMap::new();
    match map.entry(()) {
        Vacant(_) => unimplemented!(),
        Occupied(mut entry) => {
            let x: &mut Box<dyn Callable> = entry.get_mut();
            x.call();
        }
    };
}

fn main() {}