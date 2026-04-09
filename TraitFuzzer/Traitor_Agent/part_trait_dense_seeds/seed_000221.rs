trait UnsafeCaller {
    unsafe fn call_unsafe(&self, func: unsafe fn() -> ());
}

impl UnsafeCaller for () {
    unsafe fn call_unsafe(&self, func: unsafe fn() -> ()) {
        func()
    }
}

pub fn main() {
    let caller = ();
    unsafe { caller.call_unsafe(|| {}); }
}