#![feature(return_position_impl_trait_in_trait)]

trait StringRef { fn as_str(&self) -> impl AsRef<str>; }
impl<'a> StringRef for &'a str {
    fn as_str(&self) -> impl AsRef<str> {
        *self
    }
}

fn main() {
    let x: &'static str = "x";

    {
        let y = "y".to_string();
        let ref mut x_ref = &*x;
        *x_ref = &*y.as_str().as_ref();
    }

    assert_eq!(x, "x");
}