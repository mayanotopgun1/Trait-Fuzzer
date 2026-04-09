use zed::BarExt;

mod zed {
    pub trait BarExt {
        fn bar(&self);
    }

    impl BarExt for () {
        fn bar(&self) {
            println!("bar");
        }
    }
}

pub fn main() { ().bar(); }