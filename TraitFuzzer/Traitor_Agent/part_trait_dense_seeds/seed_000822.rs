#![feature(type_alias_impl_trait)]

use std::env;
use std::process::{Command, Stdio};
use std::str;

trait Panicable {
    fn double_panic(&self);
}

impl Panicable for () {
    fn double_panic(&self) {
        panic!("once");
        panic!("again");
    }
}

#[inline(never)]
fn foo() {
    let _v = vec![1, 2, 3];
    if env::var_os("IS_TEST").is_some() {
        panic!()
    }
}

#[inline(never)]
fn double() {
    Panicable::double_panic(&());
}

type HiddenCommand = Command;
fn template(me: &str) -> HiddenCommand {
    let mut m = Command::new(me);
    m.env("IS_TEST", "1")
     .stdout(Stdio::piped())
     .stderr(Stdio::piped());
    return m;
}

trait BacktraceChecker {
    fn expected(&self, fn_name: &str) -> String;
    fn contains_verbose_expected(&self, s: &str, fn_name: &str) -> bool;
}

impl BacktraceChecker for () {
    fn expected(&self, fn_name: &str) -> String {
        format!(" backtrace::{}", fn_name)
    }

    fn contains_verbose_expected(&self, s: &str, fn_name: &str) -> bool {
        let prefix = " backtrace";
        let suffix = &format!("::{}", fn_name);
        s.match_indices(prefix).any(|(i, _)| {
            s[i + prefix.len()..]
                .trim_start_matches('[')
                .trim_start_matches(char::is_alphanumeric)
                .trim_start_matches('_')
                .starts_with(suffix)
        })
    }
}

fn runtest(me: &str) {
    let p = template(me).arg("fail").env("RUST_BACKTRACE","1").spawn().unwrap();
    let out = p.wait_with_output().unwrap();
    assert!(!out.status.success());
    let s = str::from_utf8(&out.stderr).unwrap();
    assert!(s.contains("stack backtrace") && s.contains(&BacktraceChecker::expected(&(), "foo")),
            "bad output: {}", s);
    assert!(s.contains(" 0:"), "the frame number should start at 0");

    let p = template(me).arg("fail").env_remove("RUST_BACKTRACE").spawn().unwrap();
    let out = p.wait_with_output().unwrap();
    assert!(!out.status.success());
    let s = str::from_utf8(&out.stderr).unwrap();
    assert!(!s.contains("stack backtrace") && !s.contains(&BacktraceChecker::expected(&(), "foo")),
            "bad output2: {}", s);

    let p = template(me).arg("fail").env("RUST_BACKTRACE","0").spawn().unwrap();
    let out = p.wait_with_output().unwrap();
    assert!(!out.status.success());
    let s = str::from_utf8(&out.stderr).unwrap();
    assert!(!s.contains("stack backtrace") && !s.contains(" - foo"),
            "bad output3: {}", s);

    #[cfg(not(panic = "abort"))]
    {
        let p = template(me).arg("double-fail").env("RUST_BACKTRACE","0").spawn().unwrap();
        let out = p.wait_with_output().unwrap();
        assert!(!out.status.success());
        let s = str::from_utf8(&out.stderr).unwrap();

        assert!(
            s.contains("stack backtrace") &&
                s.contains("panic in a destructor during cleanup") &&
                BacktraceChecker::contains_verbose_expected(&(), s, "double"),
            "bad output3: {}", s
        );

        assert_eq!(s.split("stack backtrace").count(), 2);

        let p = template(me).arg("double-fail")
                                    .env("RUST_BACKTRACE", "1").spawn().unwrap();
        let out = p.wait_with_output().unwrap();
        assert!(!out.status.success());
        let s = str::from_utf8(&out.stderr).unwrap();
        let mut i = 0;
        for _ in 0..2 {
            i += s[i + 10..].find("stack backtrace").unwrap() + 10;
        }
        assert!(s[i + 10..].find("stack backtrace").is_none(),
                "bad output4: {}", s);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && args[1] == "fail" {
        foo();
    } else if args.len() >= 2 && args[1] == "double-fail" {
        double();
    } else {
        runtest(&args[0]);
    }
}