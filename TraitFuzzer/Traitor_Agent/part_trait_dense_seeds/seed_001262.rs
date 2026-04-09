#![feature(never_patterns)]
#![allow(incomplete_features)]

#[derive(Copy, Clone)]
enum Void {}

trait ResultAccess {
    type Value;
    fn get_value(self) -> Option<Self::Value>;
}

impl<T> ResultAccess for Result<T, Void> {
    type Value = T;
    fn get_value(self) -> Option<Self::Value> {
        match self {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}

fn main() {
    let res_void: Result<bool, Void> = Ok(true);

    if let Some(x) = res_void.get_value() {
        println!("{x}");
    } else {
        unreachable!();
    }

    if let Some(ref x) = &res_void.get_value() {
        println!("{x}");
    } else {
        unreachable!();
    }

    if let Some(x) = res_void.get_value() {
        println!("{x}");
    } else {
        unreachable!();
    }

    match res_void.get_value() {
        Some(x) => println!("{x}"),
        None => unreachable!(),
    }

    match res_void.get_value() {
        Some(x) => println!("{x}"),
        None => unreachable!(),
    }

    let res_res_void: Result<Result<bool, Void>, Void> = Ok(Ok(true));

    if let Some(x) = res_res_void.get_value().and_then(|r| r.get_value()) {
        println!("{x}");
    } else {
        unreachable!();
    }
}