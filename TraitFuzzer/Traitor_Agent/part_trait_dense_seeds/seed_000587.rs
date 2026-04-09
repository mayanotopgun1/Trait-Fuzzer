use std::marker::PhantomData;

#[derive(Default)]
struct MyType<'a> {
    field: usize,
    _phantom: PhantomData<&'a ()>,
}

#[derive(Default)]
struct MyTypeVariant<'a> {
    field: usize,
    _phantom: PhantomData<&'a ()>,
}

trait AsVariantTrait {
    type Type;
}

impl<'a> AsVariantTrait for MyType<'a> {
    type Type = MyTypeVariant<'a>;
}

type Variant<G> = <G as AsVariantTrait>::Type;

trait FieldAccess {
    fn get_field(&self) -> usize;
}

impl<'a> FieldAccess for MyType<'a> {
    fn get_field(&self) -> usize {
        self.field
    }
}

impl<'a> FieldAccess for MyTypeVariant<'a> {
    fn get_field(&self) -> usize {
        self.field
    }
}

fn foo<T: Default + FieldAccess, F: FnOnce(T)>(f: F) {
    let input = T::default();
    f(input);
}

fn main() {
    foo(|a: Variant<MyType>| {
        a.get_field();
    });
}