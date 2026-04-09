use std::any::Any;

trait Callable {
    fn call(&self, arg: &u8);
}

impl Callable for fn(&u8) {
    fn call(&self, arg: &u8) {
        self(arg)
    }
}

fn foo(_: &u8) {
}

fn main() {
    let callable = foo as fn(&u8);
    let trait_obj: Box<dyn Any> = Box::new(callable);
    let _ = trait_obj.downcast_ref::<fn(&u8)>();
}