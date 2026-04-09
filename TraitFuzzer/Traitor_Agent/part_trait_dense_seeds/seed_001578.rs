#![feature(specialization)]

static FN_1: &str = "this_function_must_be_in_the_backtrace";
static FN_2: &str = "and_this_function_too";

trait Traceable {
    fn trace(&self);
}

default impl<T> Traceable for T {
    fn trace(&self) {}
}

impl Traceable for String {
    fn trace(&self) {

    }
}

trait TraceableExt: Traceable {
    fn extended_trace(&self) -> String;
}

impl<T: Traceable + std::fmt::Debug> TraceableExt for T {
    default fn extended_trace(&self) -> String {
        format!("Extended trace for {:?}", self)
    }
}

fn this_function_must_be_in_the_backtrace() {
    and_this_function_too();
}

fn and_this_function_too() {
    panic!("generate panic backtrace");
}

trait BacktraceAssertion {
    fn assert_backtrace(&self, backtrace: &str);
}

impl BacktraceAssertion for str {
    fn assert_backtrace(&self, backtrace: &str) {
        assert!(
            backtrace.contains(self),
            "ERROR: no `{}` in stderr! actual stderr: {}",
            self,
            backtrace
        );
    }
}

fn run_test() {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("whatever")
        .env("RUST_BACKTRACE", "full")
        .output()
        .unwrap();
    let backtrace = std::str::from_utf8(&output.stderr).unwrap();

    FN_1.assert_backtrace(backtrace);
    FN_2.assert_backtrace(backtrace);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        run_test();
    } else {
        this_function_must_be_in_the_backtrace();
    }
}