struct Invariant<T>(*mut T);

trait Opaque<'a> {
    fn opaque(_: &'a str) -> Self;
}

impl<'a> Opaque<'a> for Invariant<()> {
    fn opaque(_: &'a str) -> Self {
        Invariant(&mut ())
    }
}

fn main() {
    let x = Invariant::<()>::opaque(&String::new());
    drop(x);
}