#![feature(unboxed_closures, fn_traits)]

trait FnOnceExt<A>: FnOnce<(A,)> {
    fn call_once_ext(self, args: (A,)) -> Self::Output;
}

impl<F, A> FnOnceExt<A> for F
where
    F: FnOnce<(A,)>,
{
    fn call_once_ext(self, args: (A,)) -> Self::Output {
        self.call_once(args)
    }
}

struct Foo;

impl<A> FnOnce<(A,)> for Foo {
    type Output = ();
    extern "rust-call" fn call_once(self, (_,): (A,)) {
    }
}

fn main() {
    let foo = Foo;
    println!("{:?}", foo.call_once_ext(("bar",)));
}