#[allow(unreachable_code)]
const _: () = 'a: while break 'a {};

trait LoopControl {
    fn control_flow(&self);
}

impl LoopControl for Box<dyn std::any::Any> {
    fn control_flow(&self) {
        if let Some(val) = self.downcast_ref::<()>() {
            'a: while break 'a {};
        }
    }
}

fn main() {
    let _ = (Box::new(()) as Box<dyn std::any::Any>).control_flow();
}