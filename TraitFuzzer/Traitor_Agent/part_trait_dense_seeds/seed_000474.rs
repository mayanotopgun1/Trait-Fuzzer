#![feature(return_position_impl_trait_in_trait)]

use zed::Bar;

mod zed {
    pub trait Bar {
        fn bar(&self) -> impl core::fmt::Debug;
    }

    impl Bar for () {
        fn bar(&self) -> impl core::fmt::Debug {
            println!("bar");
            0i32
        }
    }
}

trait BarExt: Bar {
    fn bar_ext(&self) -> impl core::fmt::Debug {
        self.bar()
    }
}
impl<T: Bar> BarExt for T {}

pub fn main() {
    let _ = <() as BarExt>::bar_ext(&());
}