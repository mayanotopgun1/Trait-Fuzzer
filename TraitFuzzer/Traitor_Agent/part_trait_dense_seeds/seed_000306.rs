trait Variable<'a> {
    type Type;
}

impl Variable<'_> for () {
    type Type = ();
}

trait Checkable<F, T>
where
    F: Fn(T),
    F: for<'a> Fn(<T as Variable<'a>>::Type),
    T: for<'a> Variable<'a>,
{
    fn check(self);
}

impl<F, T> Checkable<F, T> for F
where
    F: Fn(T),
    F: for<'a> Fn(<T as Variable<'a>>::Type),
    T: for<'a> Variable<'a>,
{
    fn check(self) {}
}

trait CheckableExt<F, T>: Checkable<F, T>
where
    F: Fn(T),
    F: for<'a> Fn(<T as Variable<'a>>::Type),
    T: for<'a> Variable<'a>,
{
    fn checked(self);
}

impl<S, F, T> CheckableExt<F, T> for S
where
    S: Checkable<F, T>,
    F: Fn(T),
    F: for<'a> Fn(<T as Variable<'a>>::Type),
    T: for<'a> Variable<'a>,
{
    fn checked(self) {
        self.check()
    }
}

fn test(arg: impl Fn(())) {
    fn fn_1(_: ()) {}
    let fn_2 = |_: ()| ();
    let fn_3 = |a| fn_1(a);
    let fn_4 = arg;

    fn_1.checked();
    fn_2.checked();
    fn_3.checked();
    fn_4.checked();
}

fn main() {}