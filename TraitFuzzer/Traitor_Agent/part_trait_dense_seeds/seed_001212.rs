#![crate_type = "lib"]
use std::path::Path;

trait FuncTrait {
    fn func(&self, check: bool, a: &Path, b: Option<&Path>) -> i32;
}

impl FuncTrait for A {
    fn func(&self, check: bool, a: &Path, b: Option<&Path>) -> i32 {
        if check {
            0i32
        } else if let Some(parent) = b.and_then(|p| p.parent()) {
            1i32
        } else {
            2i32
        }
    }
}

struct A {
    pub func: fn(check: bool, a: &Path, b: Option<&Path>) -> i32,
}

const MY_A: A = A {
    func: |check, a, b| {
        if check {
            0i32
        } else if let Some(parent) = b.and_then(|p| p.parent()) {
            1i32
        } else {
            2i32
        }
    },
};

fn main() {
    let a = MY_A;
    let _ = a.func(false, Path::new("test"), Some(Path::new("example")));
}