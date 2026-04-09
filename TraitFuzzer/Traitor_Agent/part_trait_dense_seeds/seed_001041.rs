#![allow(unused_mut)]

trait Boxable {
    type Output<'a> where Self: 'a;
    fn new(value: i32) -> Self::Output<'static>;
}

trait BoxableExt: Boxable {
    fn boxed_new(value: i32) -> Self::Output<'static> {
        Self::new(value)
    }
}

impl<T> BoxableExt for T where T: Boxable {}

impl<T> Boxable for T
where
    T: From<i32>,
{
    type Output<'a> = &'a T where T: 'a;

    fn new(value: i32) -> Self::Output<'static> {
        Box::leak(Box::new(value.into()))
    }
}

pub fn main() {
    let mut i: &i32 = <i32 as BoxableExt>::boxed_new(100);
    assert_eq!(*i, 100);
}