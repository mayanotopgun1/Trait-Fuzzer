#![deny(missing_copy_implementations)]

trait EnumTrait {
    fn is_a(&self) -> bool;
}

impl EnumTrait for MyEnum {
    fn is_a(&self) -> bool {
        matches!(self, MyEnum::A)
    }
}

impl EnumTrait for MyEnum2 {
    fn is_a(&self) -> bool {
        matches!(self, MyEnum2::A)
    }
}

trait StructTrait {
    fn get_foo(&self) -> usize;
}

impl StructTrait for MyStruct {
    fn get_foo(&self) -> usize {
        self.foo
    }
}

#[non_exhaustive]
pub enum MyEnum {
    A,
}

#[non_exhaustive]
pub struct MyStruct {
    foo: usize,
}

pub enum MyEnum2 {
    #[non_exhaustive]
    A,
    B,
}

fn main() {}