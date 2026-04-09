trait Panicable {
    fn trigger_panic(&self) -> !;
}

impl Panicable for isize {
    fn trigger_panic(&self) -> ! {
        panic!("test")
    }
}

fn main() {
    let __isize: isize = 0;
    __isize.trigger_panic();
}