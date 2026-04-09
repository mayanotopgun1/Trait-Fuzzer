#![allow(unused_macros)]
macro_rules! foo { ($x:pat | $y:pat) => {} }
macro_rules! bar { ($($x:pat)+ | $($y:pat)+) => {} }
macro_rules! qux { ($x:pat, $y:pat) => {} }

trait MatchAny {
    fn match_any<F>(&self, arms: F) -> i64
    where
        F: Fn(Result<i64, i32>) -> i64;
}

impl MatchAny for Result<i64, i32> {
    fn match_any<F>(&self, arms: F) -> i64
    where
        F: Fn(Result<i64, i32>) -> i64,
    {
        arms(*self)
    }
}

macro_rules! match_any {
    ( $expr:expr , $( $( $pat:pat )|+ => $expr_arm:expr ),+ ) => {
        $expr.match_any(|result| {
            match result {
                $(
                    $( $pat => $expr_arm, )+
                )+
            }
        })
    };
}

fn main() {
    let result: Result<i64, i32> = Err(42);
    let int: i64 = match_any!(result, Ok(i) | Err(i) => i.into());
    assert_eq!(int, 42);
}