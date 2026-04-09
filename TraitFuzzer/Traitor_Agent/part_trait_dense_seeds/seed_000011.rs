#![feature(specialization)]
#![allow(dead_code)]

struct S {
    f0: String,
    f1: String,
}

trait CloneString { fn clone_string(&self) -> String; }
default impl<T> CloneString for T {
    fn clone_string(&self) -> String {
        "default".to_string()
    }
}
impl CloneString for S {
    fn clone_string(&self) -> String {
        self.f0.to_string()
    }
}

pub fn main() {
    let s = "Hello, world!".to_string();
    let s = S {
        f1: s.clone(),
        f0: s
    };
    assert_eq!(s.clone_string(), "Hello, world!");
}