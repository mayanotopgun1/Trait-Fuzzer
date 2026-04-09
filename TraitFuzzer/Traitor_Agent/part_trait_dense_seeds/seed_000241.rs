use std::sync::Arc;

trait Command {
    fn execute(&self) -> i32;
}

impl<F> Command for F
where
    F: Fn() -> i32,
{
    fn execute(&self) -> i32 {
        self()
    }
}

fn main() {
    let x = 5;
    let command = Arc::new(Box::new(|| x * 2));
    assert_eq!(command.execute(), 10);
}