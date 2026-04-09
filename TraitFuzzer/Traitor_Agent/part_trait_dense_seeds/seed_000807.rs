use std::sync::atomic::{AtomicUsize, Ordering};

trait Load {
    fn load(&self, ordering: Ordering) -> usize;
}

impl Load for AtomicUsize {
    fn load(&self, ordering: Ordering) -> usize {
        self.load(ordering)
    }
}

fn exec(loader: Box<dyn Load>) -> usize {
    loader.load(Ordering::Relaxed)
}

fn main() {
    let atomic = AtomicUsize::new(0);
    let _ = exec(Box::new(atomic));
}