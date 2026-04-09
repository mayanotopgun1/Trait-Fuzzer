#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub trait bomb { fn boom(&self, _: Ident); }
trait bombExt: bomb {
    fn double_boom(&self) {
        self.boom(Ident_new());
        self.boom(Ident_new());
    }
}
impl<T: ?Sized + bomb> bombExt for T {}

pub struct S;
impl bomb for S { fn boom(&self, _: Ident) { } }

pub struct Ident { name: usize }

macro_rules! int3 { () => ( { } ) }

fn Ident_new() -> Ident {
    int3!();
    Ident {name: 0x6789ABCD }
}

trait bombUtil {
    fn use_fuse(&self, fld: Box<dyn bomb>) {
        let f = || {
            int3!();
            fld.double_boom();
        };
        f();
    }
}
impl<T> bombUtil for T {}

pub fn main() {
    let b = Box::new(S) as Box<dyn bomb>;
    let util = S;
    util.use_fuse(b);
}