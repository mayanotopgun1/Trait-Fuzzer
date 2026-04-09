#![allow(unused_must_use)]

trait Evaluate {
    fn eval(&self) -> i32;
}

trait EvaluateExt: Evaluate {
    fn eval_twice(&self) -> i32 {
        self.eval() + self.eval()
    }
}

impl<T: Evaluate> EvaluateExt for T {}

impl Evaluate for bool {
    fn eval(&self) -> i32 {
        if *self { 1 } else { 2 }
    }
}

fn main() {
    let condition = true;

    let result1 = condition.eval_twice();
    let closure_result = || condition.eval_twice();

    closure_result();
}