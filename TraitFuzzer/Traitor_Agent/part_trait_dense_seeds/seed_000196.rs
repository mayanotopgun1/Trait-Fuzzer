type A = &'static [usize; 1];
type B = &'static [usize; 100];

type DynSomething = dyn Something<Assoc = A>;

trait Super {
    type Assoc;
}
impl Super for Foo {
    type Assoc = A;
}

trait IsDynSomething {}
impl IsDynSomething for DynSomething {}

impl<T: ?Sized> Super for T
where
    T: IsDynSomething,
{
    type Assoc = B;
}

trait Something: Super {
    fn method(&self) -> Self::Assoc;
}

struct Foo;

trait FooTrait {
    fn foo_method(&self) -> A;
}
impl FooTrait for Foo {
    fn foo_method(&self) -> A {
        &[1337]
    }
}

impl Something for Foo {
    fn method(&self) -> Self::Assoc {
        self.foo_method()
    }
}

fn main() {
    let x = &Foo;
    let y: &DynSomething = x;

    let _arr1: A = x.method();

    let _arr2: A = y.method();
}