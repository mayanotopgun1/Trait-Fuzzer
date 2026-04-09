#![feature(specialization)]

use std::alloc::{Layout, handle_alloc_error};
use std::process::Command;
use std::{env, str};

trait AllocHandler {
    fn handle_alloc_error(layout: Layout) -> !;
}

default impl<T> AllocHandler for T {
    default fn handle_alloc_error(layout: Layout) -> ! {
        // Default implementation that does nothing
        unreachable!()
    }
}

impl AllocHandler for () {
    fn handle_alloc_error(layout: Layout) -> ! {
        handle_alloc_error(layout)
    }
}

trait CommandRunner {
    fn run_command(&self, args: &[&str]) -> std::process::Output;
}

default impl<T> CommandRunner for T {
    default fn run_command(&self, _args: &[&str]) -> std::process::Output {
        // Default implementation that does nothing
        unreachable!()
    }
}

impl CommandRunner for String {
    fn run_command(&self, args: &[&str]) -> std::process::Output {
        Command::new(self).args(args).env("RUST_BACKTRACE", "1").output().unwrap()
    }
}

trait StdErrProcessor {
    fn process_stderr(&mut self, stderr: &mut &str);
}

default impl<T> StdErrProcessor for T {
    default fn process_stderr(&mut self, _stderr: &mut &str) {
        // Default implementation that does nothing
        unreachable!()
    }
}

impl StdErrProcessor for () {
    fn process_stderr(&mut self, stderr: &mut &str) {
        *stderr = stderr
            .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
            .unwrap_or(*stderr);
    }
}

fn main() {
    let alloc_handler: () = ();
    if env::args().len() > 1 {
        <() as AllocHandler>::handle_alloc_error(Layout::new::<[u8; 42]>());
    }

    let me = env::current_exe().unwrap();
    let command_runner: String = me.to_string_lossy().into_owned();
    let output = command_runner.run_command(&["next"]);
    assert!(!output.status.success(), "{:?} is a success", output.status);

    let mut stderr = str::from_utf8(&output.stderr).unwrap();
    let mut stderr_processor: () = ();
    stderr_processor.process_stderr(&mut stderr);

    assert!(stderr.contains("memory allocation of 42 bytes failed"), "{}", stderr);
    assert!(stderr.contains("alloc_error_backtrace::main"), "{}", stderr);
}