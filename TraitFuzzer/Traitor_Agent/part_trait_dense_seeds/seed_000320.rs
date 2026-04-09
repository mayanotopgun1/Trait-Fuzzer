trait FooTrait<K> where Self: Sized {
    fn foo(self, x: Option<K>);
}

trait FooWrapper<K>: FooTrait<K> {}
impl<T, K> FooWrapper<K> for T where T: FooTrait<K> {}

impl<K> FooTrait<K> for () where Option<K>: Sized {
    fn foo(self, x: Option<K>) {
        let _y = x;
    }
}

fn main() {
    let _unit = ();
    _unit.foo(Some(22));
}