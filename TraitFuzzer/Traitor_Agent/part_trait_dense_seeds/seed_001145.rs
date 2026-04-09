#![allow(non_snake_case)]

use std::rc::Rc;

struct Struct { }

trait TakeSelf<'a> {
    fn take_self(self, f: &'a u32) -> &'a u32;
}

impl<'a> TakeSelf<'a> for Struct {
    fn take_self(self, f: &'a u32) -> &'a u32 {
        f
    }
}

trait TakeRefSelf<'a> {
    fn take_Self(&self, f: &'a u32) -> &'a u32;
}

impl<'a> TakeRefSelf<'a> for Struct {
    fn take_Self(&self, f: &'a u32) -> &'a u32 {
        f
    }
}

trait TakeBoxSelf<'a> {
    fn take_Box_Self(self, f: &'a u32) -> &'a u32;
}

impl<'a> TakeBoxSelf<'a> for Box<Struct> {
    fn take_Box_Self(self, f: &'a u32) -> &'a u32 {
        f
    }
}

trait TakeBoxBoxSelf<'a> {
    fn take_Box_Box_Self(self, f: &'a u32) -> &'a u32;
}

impl<'a> TakeBoxBoxSelf<'a> for Box<Box<Struct>> {
    fn take_Box_Box_Self(self, f: &'a u32) -> &'a u32 {
        f
    }
}

trait TakeRcSelf<'a> {
    fn take_Rc_Self(self, f: &'a u32) -> &'a u32;
}

impl<'a> TakeRcSelf<'a> for Rc<Struct> {
    fn take_Rc_Self(self, f: &'a u32) -> &'a u32 {
        f
    }
}

trait TakeBoxRcSelf<'a> {
    fn take_Box_Rc_Self(self, f: &'a u32) -> &'a u32;
}

impl<'a> TakeBoxRcSelf<'a> for Box<Rc<Struct>> {
    fn take_Box_Rc_Self(self, f: &'a u32) -> &'a u32 {
        f
    }
}

fn main() { }