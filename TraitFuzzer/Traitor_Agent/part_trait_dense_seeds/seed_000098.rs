use std::sync::atomic::{AtomicUsize, Ordering};

static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct A(i32);

impl Drop for A {
    fn drop(&mut self) {
        DROP_COUNTER.fetch_add(1, Ordering::SeqCst);
    }
}

static FOO: A = A(123);
const BAR: A = A(456);

trait ValueAccess { fn value(&self) -> i32; }
impl ValueAccess for A { fn value(&self) -> i32 { self.0 } }

impl A {
    const BAZ: A = A(789);
}

fn main() {
    assert_eq!(DROP_COUNTER.load(Ordering::SeqCst), 0);
    assert_eq!(FOO.value(), 123);
    assert_eq!(DROP_COUNTER.load(Ordering::SeqCst), 0);
    assert_eq!(BAR.value(), 456);
    assert_eq!(DROP_COUNTER.load(Ordering::SeqCst), 1);
    assert_eq!(A::BAZ.value(), 789);
    assert_eq!(DROP_COUNTER.load(Ordering::SeqCst), 2);
}