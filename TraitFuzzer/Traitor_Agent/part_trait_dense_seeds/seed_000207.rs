trait CheckResult {
    fn is_success(&self) -> bool;
}

impl<T> CheckResult for Result<T, Box<isize>> {
    fn is_success(&self) -> bool {
        self.is_ok()
    }
}

static C: Result<(), Box<isize>> = Ok(());

pub fn main() {
    assert!(C.is_success());
}