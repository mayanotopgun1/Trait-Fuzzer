trait Abs {
    fn abs(self) -> Self;
}

impl Abs for i32 {
    fn abs(self) -> Self {
        self.abs()
    }
}

macro_rules! foo {
    () => {
        #[allow(unreachable_patterns)]
        {
            123i32
        }
    };
}

fn compute_abs() -> i32 {
    foo!()
}

fn main() {
    let value = compute_abs();
    let _ = <i32 as Abs>::abs(value);
}