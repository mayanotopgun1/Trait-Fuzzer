#![feature(rustc_attrs)]

struct Cat {
    name: String,
}

trait DropNotify {
    fn drop_notify(&mut self);
}

impl Drop for Cat {
    #[rustc_dummy]
    fn drop(&mut self) {
        self.drop_notify();
    }
}

impl DropNotify for Cat {
    #[rustc_dummy]
    fn drop_notify(&mut self) {
        println!("{} landed on hir feet", self.name);
    }
}

trait CatFactory {
    fn create(name: String) -> Self;
}

impl CatFactory for Cat {
    #[rustc_dummy]
    fn create(name: String) -> Self {
        Cat { name }
    }
}

fn main() {
    let _kitty = Cat::create("Spotty".to_string());
}