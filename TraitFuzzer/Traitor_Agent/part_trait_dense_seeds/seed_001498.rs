trait Foo<'a, A>: Iterator<Item=A> {
    fn bar<const N: usize>(&mut self) -> *const [A; N];
}

impl<'a, A, I: ?Sized> Foo<'a, A> for I where I: Iterator<Item=A>  {
    fn bar<const N: usize>(&mut self) -> *const [A; N] {
        std::ptr::null()
    }
}

trait FooExt<'a, A>: Foo<'a, A> {
    fn foo_bar<const M: usize>(&mut self) -> *const [A; M];
}

impl<'a, A, I: ?Sized + Foo<'a, A>> FooExt<'a, A> for I {
    fn foo_bar<const M: usize>(&mut self) -> *const [A; M] {
        self.bar::<M>()
    }
}

fn main() {
    (0_u8 .. 10).foo_bar::<10_usize>();
}