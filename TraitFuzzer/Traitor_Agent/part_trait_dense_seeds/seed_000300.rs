trait Bump { fn bump() -> Option<usize>; }
impl Bump for () { fn bump() -> Option<usize> { unreachable!() } }

trait TakeUntil {
    fn take_until(terminate: impl Fn() -> bool);
}

impl TakeUntil for () {
    fn take_until(terminate: impl Fn() -> bool) {
        loop {
            if terminate() {
                return;
            } else {
                <Self as Bump>::bump();
            }
        }
    }
}

fn main() {
    <() as TakeUntil>::take_until(|| true);
    f(None);
}

fn f(_a: Option<String>) -> Option<u32> {
    loop {
        g();
        ()
    }
}

fn g() -> Option<u32> { None }