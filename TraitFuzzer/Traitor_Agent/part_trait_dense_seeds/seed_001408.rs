struct Foo<const N: usize>;

trait FooTrait<const N: usize> {
    fn foo_method(&self);
}

impl<const N: usize> FooTrait<N> for Foo<N> {
    fn foo_method(&self) {}
}

fn bindingp() {
    match Foo::<3> {
        mut x @ Foo::<3> => {
            let ref mut _x @ Foo::<3> = x;
            _x.foo_method();
        }
    }
}

struct Bar<const N: usize> {
    field: Foo<N>,
}

trait BarTrait<const N: usize>: FooTrait<N> {
    fn bar_method(&self);
}

impl<const N: usize> BarTrait<N> for Bar<N> where Bar<N>: FooTrait<N> {
    fn bar_method(&self) {}
}

fn structp() {
    match todo!() {
        Bar::<3> {
            field: _,
        } => (),
    }
}

struct Baz<const N: usize>(Foo<N>);

trait BazTrait<const N: usize>: FooTrait<N> {
    const ASSOC: usize;
}

impl<const N: usize> BazTrait<N> for Baz<N> where Baz<N>: FooTrait<N> {
    const ASSOC: usize = 3;
}

fn main() {}