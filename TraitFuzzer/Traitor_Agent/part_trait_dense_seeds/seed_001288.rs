struct Droppable(u8);

trait DropExt {
    fn drop_ext(&mut self);
}

impl Drop for Droppable {
    fn drop(&mut self) {
        eprintln!("{} dropped", self.0);
    }
}

impl DropExt for Droppable {
    fn drop_ext(&mut self) {

    }
}

trait ConvergingFn {
    fn converging_fn(&self);
}

impl ConvergingFn for () {
    fn converging_fn(&self) {
        eprintln!("converging_fn called");
    }
}

trait ExtendedDropExt: DropExt {
    fn extended_drop_ext(&mut self) {
        self.drop_ext();
    }
}

impl<T: DropExt> ExtendedDropExt for T {}

fn mir(mut d: Box<dyn ExtendedDropExt>, u: &dyn ConvergingFn) {
    d.extended_drop_ext();
    u.converging_fn();
}

fn main() {
    let d = Droppable(0);
    mir(Box::new(d), &());
    panic!("exit");
}