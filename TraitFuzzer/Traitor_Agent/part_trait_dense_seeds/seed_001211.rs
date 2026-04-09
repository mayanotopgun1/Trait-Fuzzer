#![crate_type = "lib"]
use std::path::Path;

trait FuncTrait {
    fn func(&self, check: bool, a: &Path, b: Option<&Path>);
}

impl FuncTrait for A {
    fn func(&self, check: bool, a: &Path, b: Option<&Path>) {
        if check {
            let _ = ();
        } else if let Some(parent) = b.and_then(|p| p.parent()) {
            let _ = ();
        }
    }
}

struct A {
    pub func: fn(check: bool, a: &Path, b: Option<&Path>),
}

const MY_A: A = A {
    func: |check, a, b| {
        if check {
            let _ = ();
        } else if let Some(parent) = b.and_then(|p| p.parent()) {
            let _ = ();
        }
    },
};

fn main() {
    let a = MY_A;
    a.func(false, Path::new("test"), Some(Path::new("example")));
}