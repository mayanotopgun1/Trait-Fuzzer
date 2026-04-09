pub type Yes = extern "sysv64" fn(&'static u8) -> !;

trait Call {
    unsafe fn call(self);
}

impl Call for *const Yes {
    unsafe fn call(self) {
        core::arch::asm!("call {}", in(reg) self, options(noreturn));
    }
}

fn main() {
    unsafe {
        let yes = &6 as *const _ as *const Yes;
        yes.call();
    }
}