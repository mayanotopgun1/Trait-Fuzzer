fn main() {
    struct X;
    trait Foo<T> {
        fn foo(&self) where (T, Option<T>): Ord {}
        fn bar(&self, x: &Option<T>) -> bool
        where Option<T>: Ord { *x < *x }
    }
    trait FooExt<T>: Foo<T> where T: Ord, Option<T>: Ord {}

    impl<T> FooExt<T> for dyn Foo<T> where T: Ord, Option<T>: Ord {}

    impl Foo<X> for () {}
    let _ = &() as &dyn Foo<X>;
}