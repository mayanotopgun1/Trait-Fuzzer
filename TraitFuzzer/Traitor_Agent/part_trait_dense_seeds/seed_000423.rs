trait FooTrait {
    fn foo(&self) -> impl core::fmt::Display;
}

impl FooTrait for () {
    fn foo(&self) -> impl core::fmt::Display {
        unsafe { &*(1_usize as *const [u8; 0] as *const [u8] as *const str) }
    }
}

fn main() {}