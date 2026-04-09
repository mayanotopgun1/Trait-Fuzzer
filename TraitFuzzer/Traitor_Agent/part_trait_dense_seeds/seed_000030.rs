trait Trait {
    fn foo(&self) -> &'static str;
}

trait TraitName: Trait {
    fn name(&self) -> &'static str { self.foo() }
}

impl<T: ?Sized> Trait for T {
    fn foo(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

impl<T: ?Sized + Trait> TraitName for T {}

fn bar<T: ?Sized>() -> impl Fn(&T) -> &'static str {
    move |t| TraitName::name(t)
}

fn main() {
    assert_eq!("i32", bar::<dyn TraitName>()(&1i32));
}