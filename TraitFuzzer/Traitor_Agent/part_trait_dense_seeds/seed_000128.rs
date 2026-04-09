pub trait FooTrait {
    unsafe fn foo(&self, x: *const i8) -> i8;
}

impl FooTrait for () {
    unsafe fn foo(&self, x: *const i8) -> i8 {
        *x.wrapping_sub(x as _).wrapping_add(x as _)
    }
}

fn main() {
    let x = 42;
    println!("{}", unsafe { FooTrait::foo(&(), &x) });
}