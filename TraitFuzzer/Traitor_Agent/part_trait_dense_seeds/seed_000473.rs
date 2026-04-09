use zed::Bar;

mod zed {
    pub trait Bar {
        fn bar(&self);
    }

    impl Bar for () {
        fn bar(&self) {
            println!("bar");
        }
    }
}

trait BarExt: Bar {
    fn bar_ext(&self) {
        self.bar()
    }
}
impl<T: Bar> BarExt for T {}

pub fn main() {
    let _ = <() as BarExt>::bar_ext(&());
}