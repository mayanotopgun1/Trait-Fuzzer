#![expect(incomplete_features)]
#![feature(explicit_tail_calls)]

trait FunctionLike {
    fn call(&self, arg: usize);
}

impl<F> FunctionLike for F
where
    F: Fn(usize),
{
    fn call(&self, arg: usize) {
        self(arg)
    }
}

fn f0(_: usize) {}
fn f1(_: usize) {}
fn f2(_: usize) {}

fn indexer(idx: usize) {
    let v: [Box<dyn FunctionLike>; 3] = [
        Box::new(f0),
        Box::new(f1),
        Box::new(f2),
    ];
    v[idx].call(idx)
}

fn main() {
    for idx in 0..3 {
        indexer(idx);
    }
}