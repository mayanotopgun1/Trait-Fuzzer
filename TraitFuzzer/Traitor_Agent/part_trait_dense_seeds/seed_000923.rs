trait SubtypeCheck {
    fn subtype_check<'a, 'b>(&self, f: for<'x, 'y> fn(&'x (), &'y ())) -> bool;
}

impl<T> SubtypeCheck for T
where
    T: Fn(&(), &()),
{
    fn subtype_check<'a, 'b>(&self, f: for<'x, 'y> fn(&'x (), &'y ())) -> bool {
        let _: for<'x> fn(&'x (), &'x ()) = f;
        let _: for<'x, 'y> fn(&'x (), &'y ()) = f;
        let _: for<'x> fn(&'x (), &'a ()) = f;
        let _: fn(&'a (), &'a ()) = f;
        true
    }
}

fn hr_subtype<'c>(f: for<'a, 'b> fn(&'a (), &'b ())) {
    SubtypeCheck::subtype_check(&f, f);
}

trait SimpleTrait {
    fn simple_check<'c>(&self, x: (&'static i32,)) -> bool;
}

impl<T> SimpleTrait for T
where
    T: Fn((&'static i32,)),
{
    fn simple_check<'c>(&self, x: (&'static i32,)) -> bool {
        let _: (&'c i32,) = x;
        true
    }
}

fn simple<'c>(x: (&'static i32,)) {
    SimpleTrait::simple_check(&|_| {}, x);
}

fn main() {
    hr_subtype(|_, _| {});
    simple((&3,));
}