pub trait ToPrimitive {
    fn to_int(&self);
}

impl ToPrimitive for isize {
    fn to_int(&self) {}
}

impl ToPrimitive for i32 {
    fn to_int(&self) {}
}

impl ToPrimitive for usize {
    fn to_int(&self) {}
}

trait Doit: ToPrimitive {
    fn doit(&self, f: &dyn Fn(&Self));
}

impl<T> Doit for T
where
    T: ToPrimitive,
{
    fn doit(&self, f: &dyn Fn(&T)) { f(self); }
}

pub fn main() {
    0.doit(&|x| { x.to_int(); });
}