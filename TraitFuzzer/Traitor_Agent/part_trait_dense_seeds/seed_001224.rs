#![feature(rustc_private)]

extern crate libc;

trait SigPipeHandler {
    fn handle_sigpipe(&self);
}

impl SigPipeHandler for &str {
    fn handle_sigpipe(&self) {
        unsafe {
            libc::signal(libc::SIGPIPE, libc::SIG_IGN);
        }
        assert_inherit_sigpipe_disposition(self);
    }
}

fn main() {
    "auxiliary/bin/assert-inherit-sig_dfl".handle_sigpipe();
    "auxiliary/bin/assert-inherit-sig_ign".handle_sigpipe();
}

fn assert_inherit_sigpipe_disposition(aux_bin: &str) {
    let mut cmd = std::process::Command::new(aux_bin);
    assert!(cmd.status().unwrap().success());
}