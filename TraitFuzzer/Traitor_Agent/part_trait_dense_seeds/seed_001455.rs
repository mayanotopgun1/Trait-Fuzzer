use std::fmt::Display;

trait CloneExt: Clone { fn clone_twice(&self) -> Self { self.clone() } }
impl<T: Clone> CloneExt for T {}

fn foo(f: impl Display + CloneExt) -> String {
    let g = f.clone();
    format!("{} + {}", f, g)
}

fn main() {
    let sum = foo(format!("22"));
    assert_eq!(sum, r"22 + 22");
}