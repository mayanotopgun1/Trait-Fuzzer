#![feature(type_alias_impl_trait)]

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

impl<T: Panicable> PanicExt for T {}

impl Panicable for () {
    type Assoc = dyn core::fmt::Debug;
    fn trigger_panic(&self) {
        panic!("test");
    }
}

fn main() {
    let x: () = ();
    x.double_trigger_panic();
}