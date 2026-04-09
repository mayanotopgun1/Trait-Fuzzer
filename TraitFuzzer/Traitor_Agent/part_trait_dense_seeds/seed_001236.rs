trait Trait {}

impl<T: Trait> Trait for &T {}
impl Trait for u32 {}

trait DoubleRefTrait: Trait {
    fn double_ref_trait(&self) {}
}
impl<T: Trait> DoubleRefTrait for T {}

fn hr_bound<T>()
where
    for<'a> &'a T: Trait,
{
}

fn foo<T>()
where
    T: Trait + DoubleRefTrait,
    for<'a> &'a &'a T: Trait,
{








    hr_bound::<&T>();
}

fn main() {}