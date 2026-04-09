#[allow(unreachable_code)]
const _: () = 'a: while break 'a {};

trait LoopControl {
    fn control_flow(&self);
}

impl LoopControl for () {
    fn control_flow(&self) {
        'a: while break 'a {};
    }
}

fn main() {
    let _ = ().control_flow();
}