trait Defaultable {
    fn new_default() -> Self;
}

impl<T: Default> Defaultable for T {
    fn new_default() -> Self {
        Self::default()
    }
}

fn _test() -> impl Defaultable {
    ()
}

fn main() {}