#![feature(unboxed_closures, fn_traits)]

struct S;

impl Fn<(i32,)> for S {
    extern "rust-call" fn call(&self, (x,): (i32,)) -> i32 {
        x * x
    }
}

impl FnMut<(i32,)> for S {
    extern "rust-call" fn call_mut(&mut self, args: (i32,)) -> i32 { self.call(args) }
}

impl FnOnce<(i32,)> for S {
    type Output = i32;
    extern "rust-call" fn call_once(self, args: (i32,)) -> i32 { self.call(args) }
}

trait CallExt<F> {
    fn call_it(&self, f: &F, x: i32) -> i32;
    fn call_it_mut(&mut self, f: &mut F, x: i32) -> i32;
    fn call_it_once(self, f: F, x: i32) -> i32;
}

impl<F> CallExt<F> for () where F: Fn(i32) -> i32 {
    fn call_it(&self, f: &F, x: i32) -> i32 {
        f(x)
    }

    fn call_it_mut(&mut self, f: &mut F, x: i32) -> i32 {
        f(x)
    }

    fn call_it_once(self, f: F, x: i32) -> i32 {
        f(x)
    }
}

fn main() {
    let x = ().call_it(&S, 22);
    let y = ().call_it_mut(&mut S, 22);
    let z = ().call_it_once(S, 22);
    assert_eq!(x, y);
    assert_eq!(y, z);
}