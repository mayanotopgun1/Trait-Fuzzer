trait Tr {
    fn foo(&self);
}

trait BarExt: Tr {
    fn bar(&self) {
        (|| { self.foo() })()
    }
}

impl<T> BarExt for T where T: Tr {}

impl Tr for () {
    fn foo(&self) {}
}

fn main() {
    ().bar();
}