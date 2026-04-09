#![feature(type_alias_impl_trait)]

use std::any::TypeId;

unsafe trait TidAble<'a>: Tid<'a> {}
trait TidExt<'a>: Tid<'a> {
    fn downcast_box(self: Box<Self>) {
        loop {}
    }
}

impl<'a, X: ?Sized + Tid<'a>> TidExt<'a> for X {}

unsafe trait Tid<'a>: 'a {}

unsafe impl<'a, T: ?Sized + TidAble<'a>> Tid<'a> for T {}

impl<'a> dyn Tid<'a> + 'a {
    fn downcast_any_box(self: Box<Self>) {
        self.downcast_box();
    }
}

unsafe impl<'a> TidAble<'a> for dyn Tid<'a> + 'a {}

fn main() {}