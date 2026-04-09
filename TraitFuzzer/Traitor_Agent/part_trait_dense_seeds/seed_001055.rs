#![feature(specialization)]

use std::process::Command;
use std::env;

trait ArgLenChecker {
    fn check_arg_len(&self) -> usize;
}

impl<T> ArgLenChecker for T {
    default fn check_arg_len(&self) -> usize {
        0
    }
}

impl ArgLenChecker for Vec<String> {
    fn check_arg_len(&self) -> usize {
        self.len()
    }
}

trait TestRunner {
    fn run_test(&self);
}

default impl<T> TestRunner for T {
    default fn run_test(&self) {}
}

impl TestRunner for () {
    fn run_test(&self) {
        let status = Command::new(&env::current_exe().unwrap())
                             .arg("foo").arg("")
                             .status().unwrap();
        assert!(status.success());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.check_arg_len();

    if len == 1 {
        ().run_test();
    } else {
        assert_eq!(len, 3);
    }
}