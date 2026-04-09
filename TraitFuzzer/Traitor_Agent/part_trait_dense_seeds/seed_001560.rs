#![feature(return_position_impl_trait_in_trait)]
#![crate_type = "lib"]

static mut G: i32 = 0;

trait Accessor {
    fn get_address(&self) -> usize;
}

impl Accessor for *mut i32 {
    fn get_address(&self) -> usize {
        unsafe { std::mem::transmute(self) }
    }
}

pub fn myfunc() -> i32 {
    let var = &raw mut G;
    match var.get_address() {
        0 => 0,
        _ => 1,
    }
}