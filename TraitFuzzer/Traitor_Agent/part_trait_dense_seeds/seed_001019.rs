trait ControlFlow {
    fn control_flow(&self);
}

impl ControlFlow for () {
    fn control_flow(&self) {
        if { if true { return; } else { return; }; } {}
    }
}

fn main() {
    let _: &dyn ControlFlow = &();
    ().control_flow();
}