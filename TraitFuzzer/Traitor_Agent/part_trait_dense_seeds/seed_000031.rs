#![allow(unused_must_use)]
#![allow(unreachable_code)]

trait Fooable { fn foo(&self) -> bool; }
impl Fooable for () { fn foo(&self) -> bool { false } }

trait Barable {
    fn bar(&self);
}
impl Barable for () {
    fn bar(&self) {
        return;
        !self.foo();
    }
}

trait Bazable {
    fn baz(&self);
}
impl Bazable for () {
    fn baz(&self) {
        return;
        if "" == "" {}
    }
}

pub fn main() {
    let unit = ();
    unit.bar();
    unit.baz();
}