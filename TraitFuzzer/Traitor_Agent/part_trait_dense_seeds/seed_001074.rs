trait Unsizeable {
    fn unsize(x: &[u8; 3]) -> &[u8];
}

impl Unsizeable for () {
    fn unsize(x: &[u8; 3]) -> &[u8] { x }
}

trait ClosureCreator {
    fn closure() -> fn();
}

impl ClosureCreator for () {
    fn closure() -> fn() { || {} }
}

trait ClosureConverter {
    fn closure2();
}

impl ClosureConverter for () {
    fn closure2() {
        (|| {}) as fn();
    }
}

trait Reifyable {
    fn reify(f: fn()) -> unsafe fn();
}

impl Reifyable for () {
    fn reify(f: fn()) -> unsafe fn() { f }
}

trait MainReifyable {
    fn reify2();
}

impl MainReifyable for () {
    fn reify2() { main as unsafe fn(); }
}

fn main() {}