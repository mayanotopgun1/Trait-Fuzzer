#![crate_name="a"]
#![crate_type = "lib"]

pub trait i<T>
{
    fn dummy(&self, t: T) -> T;
}

impl<T> i<T> for () {
    fn dummy(&self, t: T) -> T { panic!() }
}

pub fn f<T>() -> Box<dyn i<T>+'static> {
    Box::new(()) as Box<dyn i<T>+'static>
}