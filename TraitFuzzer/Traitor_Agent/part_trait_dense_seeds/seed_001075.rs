#![feature(generic_associated_types)]

trait Unsizeable<'a> {
    type Out;
    fn unsize(x: &'a [u8; 3]) -> Self::Out;
}

impl<'a> Unsizeable<'a> for () {
    type Out = &'a [u8];
    fn unsize(x: &'a [u8; 3]) -> Self::Out { x }
}

trait ClosureCreator {
    type Out;
    fn closure() -> Self::Out;
}

impl ClosureCreator for () {
    type Out = fn();
    fn closure() -> Self::Out { || {} }
}

trait ClosureConverter {
    type Out;
    fn closure2() -> Self::Out;
}

impl ClosureConverter for () {
    type Out = unsafe fn();
    fn closure2() -> Self::Out {
        (|| {}) as unsafe fn()
    }
}

trait Reifyable {
    type Out;
    fn reify(f: fn()) -> Self::Out;
}

impl Reifyable for () {
    type Out = unsafe fn();
    fn reify(f: fn()) -> Self::Out { f }
}

trait MainReifyable {
    type Out;
    fn reify2() -> Self::Out;
}

impl MainReifyable for () {
    type Out = unsafe fn();
    fn reify2() -> Self::Out { main as unsafe fn() }
}

fn main() {}