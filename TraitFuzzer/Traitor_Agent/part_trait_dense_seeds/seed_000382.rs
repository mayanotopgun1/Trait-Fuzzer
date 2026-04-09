#![feature(impl_trait_in_assoc_type)]

pub type Yes = extern "sysv64" fn(&'static u8) -> !;

trait Call {
    unsafe fn call(self);
}

impl Call for Box<dyn Fn(*const Yes)> {
    unsafe fn call(self) {
        (self)(std::ptr::null());
    }
}

fn main() {
    unsafe {
        let yes = &6 as *const _ as *const Yes;
        let boxed_fn: Box<dyn Fn(*const Yes)> = Box::new(move |_| core::arch::asm!("call {}", in(reg) yes, options(noreturn)));
        boxed_fn.call();
    }
}