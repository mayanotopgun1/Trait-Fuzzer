use std::sync::atomic::{AtomicUsize, Ordering};

trait Foo {
    fn store(&self, ordering: Ordering);
}

trait FooStore: Foo {
    fn store_twice(&self, ordering: Ordering) {
        self.store(ordering);
        self.store(ordering);
    }
}

impl<T> FooStore for T where T: Foo {}

impl Foo for AtomicUsize {
    fn store(&self, _ordering: Ordering) {
        AtomicUsize::store(self, 4, Ordering::SeqCst);
    }
}

fn main() -> std::process::ExitCode {
    let x = AtomicUsize::new(3);
    x.store_twice(Ordering::Acquire);
    std::process::ExitCode::SUCCESS
}