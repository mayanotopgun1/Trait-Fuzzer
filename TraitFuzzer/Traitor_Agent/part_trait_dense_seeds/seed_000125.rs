#![allow(dead_code)]

struct Foo {
    foo: i32,
    bar: i32,
    baz: (),
}

trait FooExt {
    fn extract_fields(self) -> (i32, i32);
}

impl FooExt for Foo {
    fn extract_fields(self) -> (i32, i32) {
        let Foo { foo, bar, baz } = self;
        return (foo, bar);
    }
}

fn main() {}