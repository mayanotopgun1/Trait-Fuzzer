#[repr(u32)]
#[allow(dead_code)]
enum Foo {
    A = 2,
    B,
}

trait Transmutable<T, U> {
    unsafe fn transmute(value: T) -> U;
}

impl Transmutable<u32, Option<Foo>> for Foo {
    unsafe fn transmute(value: u32) -> Option<Foo> {
        std::mem::transmute::<u32, Option<Foo>>(value)
    }
}

fn main() {
    let _val: Option<Foo> = unsafe { Foo::transmute(2) };
    let _val: Option<Foo> = unsafe { Foo::transmute(3) };
}