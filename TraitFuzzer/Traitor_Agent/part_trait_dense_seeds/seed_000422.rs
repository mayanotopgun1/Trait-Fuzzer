trait FooTrait {
    fn foo(&self) -> &str;
}

impl FooTrait for () {
    fn foo(&self) -> &str {
        unsafe { &*(1_usize as *const [u8; 0] as *const [u8] as *const str) }
    }
}

fn main() {}