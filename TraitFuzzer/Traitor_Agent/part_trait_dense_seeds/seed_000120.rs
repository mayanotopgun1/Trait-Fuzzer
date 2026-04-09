trait Fooable {
    fn foo(&mut self) -> bool;
}

trait FooableExt: Fooable {
    fn foo_twice(&mut self) -> bool {
        self.foo() && self.foo()
    }
}

impl<T: Fooable> FooableExt for T {}

impl Fooable for i32 {
    fn foo(&mut self) -> bool {
        true
    }
}

fn main() {
    let opt = Some(92);
    let mut x = 62;

    if let Some(_) = opt {

    } else if x.foo_twice() {

    }
}