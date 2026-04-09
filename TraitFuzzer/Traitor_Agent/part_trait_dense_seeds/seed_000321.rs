trait FooTrait<K> where Self: Sized {
    type Out<'a, T> where Self: 'a;
    fn foo<'a, T>(self, x: Option<K>) -> Self::Out<'a, T>;
}

trait FooWrapper<K>: FooTrait<K> {}
impl<T, K> FooWrapper<K> for T where T: FooTrait<K> {}

impl<K> FooTrait<K> for () where Option<K>: Sized {
    type Out<'a, T> = &'a ();
    fn foo<'a, T>(self, x: Option<K>) -> Self::Out<'a, T> {
        let _y = x;
        &()
    }
}

fn main() {
    let _unit = ();
    let _result: &() = _unit.foo::<()>(Some(22));
}