#![feature(return_position_impl_trait_in_trait)]
#![feature(rustc_private)]

extern crate libc;

trait SigPipeHandler {
    fn handle_sigpipe(&self) -> impl Fn();
}

impl SigPipeHandler for &str {
    fn handle_sigpipe(&self) -> impl Fn() {
        move || {
            unsafe {
                libc::signal(libc::SIGPIPE, libc::SIG_IGN);
            }
            assert_inherit_sigpipe_disposition(self);
        }
    }
}

fn main() {
    let handler1 = "auxiliary/bin/assert-inherit-sig_dfl".handle_sigpipe();
    let handler2 = "auxiliary/bin/assert-inherit-sig_ign".handle_sigpipe();
    handler1();
    handler2();
}

fn assert_inherit_sigpipe_disposition(aux_bin: &str) {
    let mut cmd = std::process::Command::new(aux_bin);
    assert!(cmd.status().unwrap().success());
}