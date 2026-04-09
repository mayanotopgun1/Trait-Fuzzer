#![allow(dead_code)]
#![feature(impl_trait_in_assoc_type)]

trait Typer<'tcx> {
    type Out;

    fn method(&self, data: &'tcx isize) -> Self::Out;
    fn dummy(&self);
}

trait TyperExt<'tcx>: Typer<'tcx> {
    fn method_twice(&self, data: &'tcx isize) -> Self::Out {
        self.method(data)
    }
}

impl<'tcx, T: ?Sized> TyperExt<'tcx> for T where T: Typer<'tcx> {}

trait TyperDummy<'tcx>: Typer<'tcx> {
    fn dummy_twice(&self) {
        self.dummy();
        self.dummy();
    }
}

impl<'tcx, T: ?Sized> TyperDummy<'tcx> for T where T: Typer<'tcx> {}

fn g<F>(_: F) where F: FnOnce(&dyn Typer<Out = &'static isize>) {}

struct DummyTyper;

impl<'tcx> Typer<'tcx> for DummyTyper {
    type Out = &'tcx isize;

    fn method(&self, data: &'tcx isize) -> Self::Out {
        data
    }

    fn dummy(&self) {}
}

fn h() {
    let typer: &dyn Typer<Out = &'static isize> = &DummyTyper;
    g(|_| typer.dummy_twice())
}

fn main() { }