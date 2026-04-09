#![warn(rust_2021_compatibility)]

#[derive(Debug)]
struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?} dropped", self.0);
    }
}

trait DebugPrint {
    fn debug_print(&self);
}

impl<T: std::fmt::Debug> DebugPrint for T {
    fn debug_print(&self) {
        println!("{:?}", self);
    }
}

macro_rules! m {
    (@ $body:expr) => {{
        let f = || $body;

        f();
    }};
    ($body:block) => {{
        m!(@ $body);
    }};
}

fn main() {
    let a = (Foo(0), Foo(1));
    m!({
        let x = a.0;
        x.debug_print();
    });
}