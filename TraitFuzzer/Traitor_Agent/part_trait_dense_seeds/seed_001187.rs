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

fn main() {
    let _ = <i32 as Abs>::abs(foo!());
}