#![feature(type_alias_impl_trait)]

use std::rc::Rc;

struct Struct { }

trait TakeSelf<'a> {
    type Out;
    fn take_self(self, f: &'a u32) -> Self::Out;
}

impl<'a> TakeSelf<'a> for Struct {
    type Out = &'a u32;
    fn take_self(self, f: &'a u32) -> Self::Out {
        f
    }
}

trait TakeRefSelf<'a> {
    type Out;
    fn take_Self(&self, f: &'a u32) -> Self::Out;
}

impl<'a> TakeRefSelf<'a> for Struct {
    type Out = &'a u32;
    fn take_Self(&self, f: &'a u32) -> Self::Out {
        f
    }
}

trait TakeBoxSelf<'a> {
    type Out;
    fn take_Box_Self(self, f: &'a u32) -> Self::Out;
}

impl<'a> TakeBoxSelf<'a> for Box<Struct> {
    type Out = &'a u32;
    fn take_Box_Self(self, f: &'a u32) -> Self::Out {
        f
    }
}

trait TakeBoxBoxSelf<'a> {
    type Out;
    fn take_Box_Box_Self(self, f: &'a u32) -> Self::Out;
}

impl<'a> TakeBoxBoxSelf<'a> for Box<Box<Struct>> {
    type Out = &'a u32;
    fn take_Box_Box_Self(self, f: &'a u32) -> Self::Out {
        f
    }
}

trait TakeRcSelf<'a> {
    type Out;
    fn take_Rc_Self(self, f: &'a u32) -> Self::Out;
}

impl<'a> TakeRcSelf<'a> for Rc<Struct> {
    type Out = &'a u32;
    fn take_Rc_Self(self, f: &'a u32) -> Self::Out {
        f
    }
}

trait TakeBoxRcSelf<'a> {
    type Out;
    fn take_Box_Rc_Self(self, f: &'a u32) -> Self::Out;
}

impl<'a> TakeBoxRcSelf<'a> for Box<Rc<Struct>> {
    type Out = &'a u32;
    fn take_Box_Rc_Self(self, f: &'a u32) -> Self::Out {
        f
    }
}

fn main() { }