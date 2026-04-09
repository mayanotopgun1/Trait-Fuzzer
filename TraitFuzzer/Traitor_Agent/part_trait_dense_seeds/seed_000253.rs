trait Foo { fn foo(&self) -> [u8; 4 * 1024 * 1024 * 1024 * 1024]; }

struct Bar;

impl Foo for Bar {
    fn foo(&self) -> [u8; 4 * 1024 * 1024 * 1024 * 1024] {
        unimplemented!()
    }
}

fn main() {
    let bar = Bar;
    bar.foo();
}