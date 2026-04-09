use std::process::Command;
use std::str;

trait CommandExecutor {
    fn execute(&self, arg: &str) -> String;
}

impl CommandExecutor for Vec<String> {
    fn execute(&self, arg: &str) -> String {
        let out = Command::new(&self[0]).env("RUST_BACKTRACE", "1").arg(arg).output().unwrap();
        format!(
            "{}\n{}",
            str::from_utf8(&out.stdout).unwrap(),
            str::from_utf8(&out.stderr).unwrap(),
        )
    }
}

trait OutputValidator {
    fn validate(&self, output: &str) -> bool;
}

impl OutputValidator for Vec<String> {
    fn validate(&self, output: &str) -> bool {
        let status = Command::new(&self[0]).env("RUST_BACKTRACE", "1").arg("foo").output().unwrap().status.success();
        status && output.contains(file!())
    }
}

trait DebugPrinter {
    fn print_debug_info(&self, out: &std::process::Output);
}

impl DebugPrinter for Vec<String> {
    fn print_debug_info(&self, out: &std::process::Output) {
        println!("status: {}", out.status);
        let output = format!(
            "{}\n{}",
            str::from_utf8(&out.stdout).unwrap(),
            str::from_utf8(&out.stderr).unwrap(),
        );
        println!("child output:\n\t{}", output.replace("\n", "\n\t"));
    }
}

#[inline(never)]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        println!("{}", std::backtrace::Backtrace::force_capture());
        return;
    }
    let output = args.execute("foo");
    if !args.validate(&output) {
        let out = Command::new(&args[0]).env("RUST_BACKTRACE", "1").arg("foo").output().unwrap();
        args.print_debug_info(&out);
        panic!("failed to find {:?} in output", file!());
    }
}