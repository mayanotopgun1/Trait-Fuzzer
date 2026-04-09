#![feature(impl_trait_in_assoc_type)]

use std::sync::Once;
use std::panic;

trait PanicGuard {
    fn guarded_call(&self, f: impl FnOnce() + Send + std::panic::UnwindSafe);
}

impl PanicGuard for Once {
    fn guarded_call(&self, f: impl FnOnce() + Send + std::panic::UnwindSafe) {
        let _ = panic::catch_unwind(|| f());
    }
}

trait CallOnceExt: PanicGuard {
    fn call_once_safe(&self, f: impl FnOnce() -> () + Sync + Send + std::panic::UnwindSafe);
}

impl<T> CallOnceExt for T
where
    T: PanicGuard,
{
    fn call_once_safe(&self, f: impl FnOnce() -> () + Sync + Send + std::panic::UnwindSafe) {
        self.guarded_call(f);
    }
}

fn main() {
    let o = Once::new();
    o.call_once_safe(|| panic!("Here Once instance is poisoned."));
    o.call_once(|| {});
}