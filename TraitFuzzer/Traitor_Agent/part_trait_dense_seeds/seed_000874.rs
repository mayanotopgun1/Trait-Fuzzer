#![feature(lazy_type_alias)]
#![allow(incomplete_features)]

type Alias = Local;

struct Local;

trait MethodTrait {
    fn method(self);
}

impl MethodTrait for Local {
    fn method(self) {}
}

fn main() {
    let _ = Local.method();
    let _ = <Local as MethodTrait>::method;
    let _ = Alias {}.method();
    let _ = <Alias as MethodTrait>::method;
}