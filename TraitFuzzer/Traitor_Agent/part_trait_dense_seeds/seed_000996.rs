#![feature(impl_trait_in_assoc_type)]

static FN_1: &str = "this_function_must_be_in_the_backtrace";
fn this_function_must_be_in_the_backtrace() {
    and_this_function_too();
}

static FN_2: &str = "and_this_function_too";
fn and_this_function_too() -> impl core::fmt::Debug {
    panic!("generate panic backtrace");
}

trait TestRunner {
    fn run_test(&self);
}

impl TestRunner for () {
    fn run_test(&self) {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .arg("whatever")
            .env("RUST_BACKTRACE", "full")
            .output()
            .unwrap();
        let backtrace = std::str::from_utf8(&output.stderr).unwrap();

        fn assert(function_name: &str, backtrace: &str) {
            assert!(
                backtrace.contains(function_name),
                "ERROR: no `{}` in stderr! actual stderr: {}",
                function_name,
                backtrace
            );
        }
        assert(FN_1, backtrace);
        assert(FN_2, backtrace);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        TestRunner::run_test(&());
    } else {
        this_function_must_be_in_the_backtrace();
    }
}