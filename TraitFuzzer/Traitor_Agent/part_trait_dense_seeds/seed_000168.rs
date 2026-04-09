macro_rules! exp {
    (const $n:expr) => {
        $n
    };
}

trait Exp {
    fn evaluate(&self) -> i32;
}

impl Exp for i32 {
    fn evaluate(&self) -> i32 {
        *self
    }
}

macro_rules! stmt {
    (exp $e:expr) => {
        ($e as &dyn Exp).evaluate()
    };
    (exp $($t:tt)+) => {
        exp!($($t)+).evaluate()
    };
}

fn main() {
    stmt!(exp const 1);
}