#![allow(unused_variables)]

#[non_exhaustive]
pub struct NormalStruct {
    pub first_field: u16,
    pub second_field: u16,
}

trait Destructure {
    fn destructure(self) -> (u16, u16);
}

impl Destructure for NormalStruct {
    fn destructure(self) -> (u16, u16) {
        (self.first_field, self.second_field)
    }
}

#[non_exhaustive]
pub struct UnitStruct;

trait UnitDestructure {
    fn unit_destructure(self);
}

impl UnitDestructure for UnitStruct {
    fn unit_destructure(self) {}
}

#[non_exhaustive]
pub struct TupleStruct(pub u16, pub u16);

trait DestructureTuple {
    fn destructure_tuple(self) -> (u16, u16);
}

impl DestructureTuple for TupleStruct {
    fn destructure_tuple(self) -> (u16, u16) {
        (self.0, self.1)
    }
}

fn main() {
    let ns = NormalStruct { first_field: 640, second_field: 480 };

    let (first_field, second_field) = ns.destructure();

    let ts = TupleStruct { 0: 340, 1: 480 };
    let ts_constructor = TupleStruct(340, 480);

    let (first, second) = ts.destructure_tuple();
    let (first, second) = ts_constructor.destructure_tuple();

    let us = UnitStruct {};
    let us_constructor = UnitStruct;

    us.unit_destructure();
}