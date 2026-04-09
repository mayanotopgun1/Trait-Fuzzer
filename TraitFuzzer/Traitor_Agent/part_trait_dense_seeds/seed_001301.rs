struct Foo;

trait Bar {
    const BAR: f32;
}

impl Bar for Foo {
    const BAR: f32 = 1.5;
}

const FOOBAR: f32 = <Foo as Bar>::BAR;

fn main() {
    assert_eq!(1.5f32, FOOBAR);
}