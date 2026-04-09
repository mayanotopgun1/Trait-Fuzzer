#[track_caller]
fn caller_line() -> u32 {
    std::panic::Location::caller().line()
}

trait CallerLineExt {
    fn get_caller_line(&self) -> u32;
}

impl CallerLineExt for () {
    fn get_caller_line(&self) -> u32 {
        caller_line()
    }
}

fn main() {
    let prev_line = caller_line();
    (A { prev_line })
    [0];
}

struct A {
    prev_line: u32,
}

impl std::ops::Index<usize> for A {
    type Output = ();

    fn index(&self, _idx: usize) -> &() {
        assert_eq!(caller_line(), self.prev_line + 2);
        &()
    }
}