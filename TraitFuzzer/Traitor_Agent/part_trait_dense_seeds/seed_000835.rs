#![feature(stmt_expr_attributes)]

trait ClosureTrait {
    fn call(&self);
}

impl<T> ClosureTrait for T
where
    T: Fn() + 'static,
{
    fn call(&self) {
        self()
    }
}

pub fn make_closure() -> impl ClosureTrait {
    || {}
}

pub fn main() {
    let _x = Box::new(make_closure()) as Box<dyn ClosureTrait>;
    let _y = Box::new(make_closure()) as Box<dyn ClosureTrait>;
    let _z = Box::new(make_closure()) as Box<dyn ClosureTrait>;

    _x.call();
    _y.call();
    _z.call();
}