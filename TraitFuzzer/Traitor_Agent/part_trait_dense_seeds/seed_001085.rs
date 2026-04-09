#![allow(unused_macro_rules)]

macro_rules! m {
    ($e:expr) => {
        "expr includes attr"
    };
    (#[$attr:meta] $e:expr) => {
        "expr excludes attr"
    }
}

macro_rules! n {
    (#[$attr:meta] $e:expr) => {
        "expr excludes attr"
    };
    ($e:expr) => {
        "expr includes attr"
    }
}

trait MacroChecker {
    fn check_m(&self, e: &str) -> &str;
    fn check_n(&self, e: &str) -> &str;
}

impl MacroChecker for () {
    fn check_m(&self, e: &str) -> &str {
        m!(#[attr] 1)
    }

    fn check_n(&self, e: &str) -> &str {
        n!(#[attr] 1)
    }
}

fn main() {
    let checker = ();
    assert_eq!(checker.check_m("attr"), "expr includes attr");
    assert_eq!(checker.check_n("attr"), "expr excludes attr");
}