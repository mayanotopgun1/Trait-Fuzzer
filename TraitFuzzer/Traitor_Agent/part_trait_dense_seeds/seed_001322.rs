trait PrintLike { fn print(&self); }

impl<T: std::fmt::Display> PrintLike for T {
    fn print(&self) {
        println!("{}", self);
    }
}

pub fn main() {
    let i: Box<dyn PrintLike> = Box::new(100);
    i.print();
}