use std::sync::atomic::{AtomicUsize, Ordering};

trait Load {
    fn load(&self, ordering: Ordering) -> usize;
}

impl Load for AtomicUsize {
    fn load(&self, ordering: Ordering) -> usize {
        self.load(ordering)
    }
}

fn main() {
    let atomic = AtomicUsize::new(0);
    let _ = atomic.load(Ordering::Relaxed);
}