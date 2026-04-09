#![expect(incomplete_features)]
#![feature(explicit_tail_calls)]

trait Become {
    fn r#become(&self);
}

impl Become for () {
    fn r#become(&self) {
        f();
    }
}

#[cfg(constant)]
const _: () = {
    ().r#become();
};

#[cfg(array)]
struct Bad([(); 1]);

fn f() {}

fn main() {}