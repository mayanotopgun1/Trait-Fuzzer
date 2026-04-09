trait FnLike<A,R> {
    fn call(&self, arg: A) -> R;
}

trait FnExt<'b>: for<'a> FnLike<(&'a i32,), &'a i32> + 'b {
    fn repeated_call<'a>(&self, value: &'a i32) -> &'a i32 {
        self.call((&value,))
    }
}

impl<'a, T> FnExt<'a> for T where T: for<'b> FnLike<(&'b i32,), &'b i32> + 'a {}

type FnObject<'b> = dyn FnExt<'b>;

struct Identity;

impl<'a, T> FnLike<(&'a T,), &'a T> for Identity {
    fn call(&self, (arg,): (&'a T,)) -> &'a T {
        arg
    }
}

fn call_repeatedly(f: &FnObject) {
    let x = 3;
    let y = f.repeated_call(&x);
    assert_eq!(3, *y);
}

fn main() {
    call_repeatedly(&Identity);
}