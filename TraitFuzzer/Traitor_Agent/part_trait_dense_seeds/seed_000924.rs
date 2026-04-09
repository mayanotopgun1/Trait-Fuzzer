trait SubtypeCheck {
    fn subtype_check(&self, f: &dyn Fn(&(), &())) -> bool;
}

impl<T> SubtypeCheck for T
where
    T: Fn(&(), &()),
{
    fn subtype_check(&self, f: &dyn Fn(&(), &())) -> bool {
        let _: &dyn Fn(&(), &()) = f;
        true
    }
}

fn hr_subtype(f: &dyn Fn(&(), &())) {
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

fn simple(x: (&'static i32,)) {
    SimpleTrait::simple_check(&|_| {}, x);
}

fn main() {
    hr_subtype(&|_, _| {});
    simple((&3,));
}