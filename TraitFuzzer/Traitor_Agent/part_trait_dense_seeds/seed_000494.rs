#![feature(generic_associated_types)]
#![allow(dead_code)]

use std::marker::PhantomData;

#[derive(Copy, Clone)]
struct Foo<T> { x: T }

type Ty<'tcx> = &'tcx TyS<'tcx>;

enum TyS<'tcx> {
    Boop(PhantomData<*mut &'tcx ()>)
}

#[derive(Copy, Clone)]
enum Bar<'tcx> {
    Baz(Foo<Ty<'tcx>>)
}

trait FooTrait<T: Copy> { fn get_x(&self) -> T; }
impl<T: Copy> FooTrait<T> for Foo<T> { fn get_x(&self) -> T { self.x } }

trait BarTrait<'tcx> {
    type Output;
    fn baz_value(&self) -> Self::Output;
}

impl<'tcx> BarTrait<'tcx> for Bar<'tcx> {
    type Output = Foo<Ty<'tcx>>;
    fn baz_value(&self) -> Self::Output {
        match self {
            Bar::Baz(foo) => *foo,
        }
    }
}

trait ExtendedBarTrait<'tcx>: BarTrait<'tcx> where Self::Output: FooTrait<Ty<'tcx>> {
    fn get_foo_x(&self) -> Ty<'tcx> {
        let foo = self.baz_value();
        foo.get_x()
    }
}

impl<'tcx> ExtendedBarTrait<'tcx> for Bar<'tcx> {}

fn main() { }