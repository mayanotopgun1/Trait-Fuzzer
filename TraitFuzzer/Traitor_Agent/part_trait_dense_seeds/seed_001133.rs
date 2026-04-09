#![allow(function_casts_as_integer)]
#![feature(rustc_private)]
#![feature(specialization)]
extern crate libc;

use libc::*;

unsafe extern "C" fn signal_handler(signum: c_int, _: *mut siginfo_t, _: *mut c_void) {
    assert_eq!(signum, SIGWINCH);
}

trait SignalHandler {
    unsafe fn handle(&self, signum: c_int, info: *mut siginfo_t, context: *mut c_void);
}

impl<T> SignalHandler for T
where
    T: Fn(c_int, *mut siginfo_t, *mut c_void),
{
    default unsafe fn handle(&self, signum: c_int, info: *mut siginfo_t, context: *mut c_void) {
        self(signum, info, context);
    }
}

impl SignalHandler for unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void) {
    unsafe fn handle(&self, signum: c_int, info: *mut siginfo_t, context: *mut c_void) {
        self(signum, info, context);
    }
}

trait SignalAction {
    fn set_action(&mut self, signum: c_int, action: &sigaction);
}

impl SignalAction for libc::sigaction {
    fn set_action(&mut self, signum: c_int, action: &sigaction) {
        unsafe { sigaction(signum, action, std::ptr::null_mut()); }
    }
}

trait ExitHandler {
    fn at_exit(&self, func: extern "C" fn());
}

impl<T> ExitHandler for T
where
    T: Fn(),
{
    default fn at_exit(&self, func: extern "C" fn()) {
        unsafe { atexit(func); }
    }
}

impl ExitHandler for libc::c_void {
    fn at_exit(&self, func: extern "C" fn()) {
        unsafe { atexit(func); }
    }
}

extern "C" fn send_signal() {
    unsafe {
        raise(SIGWINCH);
    }
}

fn main() {
    unsafe {

        let mut action: sigaction = std::mem::zeroed();
        action.sa_flags = (SA_ONSTACK | SA_SIGINFO) as _;
        action.sa_sigaction = signal_handler as sighandler_t;

        let mut old_action: sigaction = std::mem::zeroed();
        old_action.set_action(SIGWINCH, &action);

        atexit(send_signal);
    }
}