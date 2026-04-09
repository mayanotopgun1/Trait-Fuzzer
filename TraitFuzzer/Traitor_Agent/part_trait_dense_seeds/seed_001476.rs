#![feature(generic_associated_types)]

trait BooleanValue {
    type Output<'a> where Self: 'a;
    fn value<'a>(&'a self) -> Self::Output<'a>;
}

trait BooleanDebug: BooleanValue {
    fn debug_value<'a>(&'a self) -> String where Self::Output<'a>: std::fmt::Debug {
        format!("{:?}", self.value())
    }
}

impl<T> BooleanDebug for T where T: BooleanValue {}

impl BooleanValue for bool {
    type Output<'a> = &'a bool;
    fn value<'a>(&'a self) -> Self::Output<'a> {
        self
    }
}

fn make_debuggable<B: BooleanDebug>(value: B) -> impl Fn() -> String where for<'a> <B as BooleanValue>::Output<'a>: std::fmt::Debug {
    move || value.debug_value()
}

fn main() {
    let inappropriate_camel_casing = true;
    let debug_fn = make_debuggable(inappropriate_camel_casing);
    println!("{}", debug_fn());
}