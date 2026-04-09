#[allow(dead_code)]
#[repr(u32)]
enum Foo {
    A,
    B,
}

trait Transmogrify<T, U> {
    unsafe fn transmute(self) -> U;
}

impl Transmogrify<Bar, Foo> for Bar {
    unsafe fn transmute(self) -> Foo {
        std::mem::transmute(self)
    }
}

#[allow(dead_code)]
struct Bar {
    a: u32,
}

fn main() {
    let bar = Bar { a: 3 };
    let _val: Foo = unsafe { bar.transmute() };
}