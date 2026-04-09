trait Boxable {
    fn box_new(self) -> Box<Self>;
}

impl<T> Boxable for T {
    fn box_new(self) -> Box<Self> {
        Box::new(self)
    }
}

fn main() {
    let _a = 1.box_new();
}