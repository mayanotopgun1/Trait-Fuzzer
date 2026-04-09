trait Fooable<'a> {
    fn foo() -> Self;
}

impl<'a> Fooable<'a> for () {
    fn foo() -> Self {
        let _: () = <Self as Fooable>::foo();
        loop {}
    }
}

fn main() {
    <() as Fooable>::foo();
}