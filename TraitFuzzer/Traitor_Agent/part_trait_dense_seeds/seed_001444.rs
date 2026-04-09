#![feature(return_position_impl_trait_in_trait)]
#![allow(unused_variables)]

macro_rules! foo {
    ($l:lifetime, $l2:lifetime) => {
        fn f<$l: 'a, $l2: 'b>(arg: &$l str, arg2: &$l2 str) -> &'a str {
            arg
        }
    }
}

trait StringAccess<'a> {
    type Output;
    fn access(&self) -> Self::Output;
}

impl<'a> StringAccess<'a> for &'a str {
    type Output = &'a str;
    fn access(&self) -> Self::Output {
        self
    }
}

trait StringAccessExt<'a>: StringAccess<'a> {
    fn extended_access(&self) -> (Self::Output, Self::Output)
    where
        Self::Output: Copy,
    {
        let v = self.access();
        (v, v)
    }
}

impl<'a, T: ?Sized + 'a> StringAccessExt<'a> for T where T: StringAccess<'a> {}

pub fn main() {
    foo!('a, 'b);
    let x: &'static str = f("hi", "there").extended_access().0;
    assert_eq!("hi", x);
}