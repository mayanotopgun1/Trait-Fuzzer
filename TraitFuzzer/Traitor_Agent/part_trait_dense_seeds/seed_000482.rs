#![crate_name="a"]
#![crate_type = "lib"]

pub trait I<T>
{
    fn dummy(&self, t: T) -> Box<dyn core::fmt::Debug>;
}

impl<T> I<T> for () {
    fn dummy(&self, t: T) -> Box<dyn core::fmt::Debug> { panic!() }
}

pub fn f<T>() -> Box<dyn I<T>+'static> {
    Box::new(())
}