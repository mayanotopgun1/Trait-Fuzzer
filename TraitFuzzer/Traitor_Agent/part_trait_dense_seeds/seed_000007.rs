#![feature(type_alias_impl_trait)]
#![feature(inherent_associated_types)]

trait Panicable {
    type Assoc: ?Sized;
    fn trigger_panic(&self);
}

trait PanicExt: Panicable {
    fn double_trigger_panic(&self) {
        self.trigger_panic();
        self.trigger_panic();
    }
}

impl<T: ?Sized + Panicable> PanicExt for T {}

impl dyn Panicable<Assoc = dyn core::fmt::Debug> {
    type Assoc = dyn core::fmt::Debug;
    fn trigger_panic(&self) {
        panic!("test");
    }
}

struct Unit;

impl Panicable for Unit {
    type Assoc = dyn core::fmt::Debug;
    fn trigger_panic(&self) {
        panic!("test");
    }
}

fn main() {
    let x: Box<dyn Panicable<Assoc = dyn core::fmt::Debug>> = Box::new(Unit);
    x.double_trigger_panic();
}