trait StringRef { fn as_str(&self) -> &'static str; }
impl StringRef for &'static str {
    fn as_str(&self) -> &'static str {
        *self
    }
}

fn main() {
    let x: &'static str = "x";

    {
        let y = "y".to_string();
        let ref mut x_ref = &*x;
        *x_ref = &*y.as_str();
    }

    assert_eq!(x, "x");
}