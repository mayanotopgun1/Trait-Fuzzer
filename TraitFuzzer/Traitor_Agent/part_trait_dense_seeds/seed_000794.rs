#![allow(dead_code)]

trait Typer<'tcx> {
    fn method(&self, data: &'tcx isize) -> &'tcx isize;
    fn dummy(&self);
}

trait TyperExt<'tcx>: Typer<'tcx> {
    fn method_twice(&self, data: &'tcx isize) -> &'tcx isize {
        self.method(data)
    }
}

impl<'tcx, T> TyperExt<'tcx> for T where T: Typer<'tcx> {}

trait TyperDummy<'tcx>: Typer<'tcx> {
    fn dummy_twice(&self) {
        self.dummy();
        self.dummy();
    }
}

impl<'tcx, T: ?Sized> TyperDummy<'tcx> for T where T: Typer<'tcx> {}

fn g<F>(_: F) where F: FnOnce(&dyn Typer) {}

struct DummyTyper;

impl<'tcx> Typer<'tcx> for DummyTyper {
    fn method(&self, data: &'tcx isize) -> &'tcx isize {
        data
    }

    fn dummy(&self) {}
}

fn h() {
    let typer: &dyn Typer = &DummyTyper;
    g(|_| typer.dummy_twice())
}

fn main() { }