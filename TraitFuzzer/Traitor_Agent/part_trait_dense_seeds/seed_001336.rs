#![expect(incomplete_features)]
#![feature(explicit_tail_calls, generic_associated_types)]

trait Become {
    type Output;
    fn r#become(&self) -> Self::Output;
}

impl Become for () {
    type Output = ();
    fn r#become(&self) -> Self::Output {
        f();
        ()
    }
}

#[cfg(constant)]
const _: () = {
    let _ = <() as Become>::r#become(&());
};

#[cfg(array)]
struct Bad([(); 0]);

trait BecomeExt: Become {
    fn become_twice(&self) -> Self::Output {
        self.r#become()
    }
}

impl<T> BecomeExt for T where T: Become {}

fn f() {}

fn main() {}