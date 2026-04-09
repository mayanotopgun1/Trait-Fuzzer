#![deny(missing_copy_implementations)]
#![feature(dyn_trait)]

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

fn main() {
    let my_enum = MyEnum::A;
    let my_enum2 = MyEnum2::A;
    let my_struct = MyStruct { foo: 42 };

    let trait_obj1: &dyn EnumTrait = &my_enum;
    let trait_obj2: &dyn EnumTrait = &my_enum2;
    let struct_trait_obj: &dyn StructTrait = &my_struct;

    println!("Is my_enum A? {}", trait_obj1.is_a());
    println!("Is my_enum2 A? {}", trait_obj2.is_a());
    println!("Foo value: {}", struct_trait_obj.get_foo());
}