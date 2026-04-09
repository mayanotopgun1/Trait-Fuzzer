struct Foo {
    x: u32
}

trait TwiddleLike {
    fn twiddle(&mut self) -> &mut Self;
}

trait TwaddleLike {
    fn twaddle(&mut self) -> &mut Self;
}

trait EmitLike {
    fn emit(&mut self);
}

impl TwiddleLike for Foo {
    fn twiddle(&mut self) -> &mut Self { self }
}

impl TwaddleLike for Foo {
    fn twaddle(&mut self) -> &mut Self { self }
}

impl EmitLike for Foo {
    fn emit(&mut self) {
        self.x += 1;
    }
}

fn main() {
    let mut foo = Foo { x: 0 };
    match 22 {
        22 => &mut foo,
        44 => foo.twiddle(),
        _ => foo.twaddle(),
    }.emit();
}