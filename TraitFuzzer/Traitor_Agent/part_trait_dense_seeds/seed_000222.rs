#![feature(return_position_impl_trait_in_trait)]

trait UnsafeCaller {
    unsafe fn call_unsafe(&self) -> impl Fn(unsafe fn() -> ());
}

impl UnsafeCaller for () {
    unsafe fn call_unsafe(&self) -> impl Fn(unsafe fn() -> ()) {
        |func| func()
    }
}

pub fn main() {
    let caller = ();
    unsafe { 
        let closure = caller.call_unsafe();
        closure(|| {});
    }
}