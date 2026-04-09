#![deny(unused_allocation)]
#![feature(generic_associated_types)]

#[derive(Clone)]
struct MyStruct;

trait TraitTakesBoxRef<'a> {
    type RefType;
    fn trait_takes_box_ref(&'a self) -> Self::RefType;
}

impl<'a> TraitTakesBoxRef<'a> for Box<MyStruct> {
    type RefType = &'a MyStruct;
    fn trait_takes_box_ref(&'a self) -> Self::RefType { &**self }
}

trait TraitExtendsTraitTakesBoxRef: for<'a> TraitTakesBoxRef<'a> {}

impl<T: for<'a> TraitTakesBoxRef<'a>> TraitExtendsTraitTakesBoxRef for T {}

impl MyStruct {
    fn inherent_takes_box_ref(self: &Box<Self>) {}
}

fn takes_box_ref(_: &Box<MyStruct>) {}

trait TraitTakesBoxVal {
    type ValType;
    fn trait_takes_box_val(self) -> Self::ValType;
}

impl TraitTakesBoxVal for Box<MyStruct> {
    type ValType = MyStruct;
    fn trait_takes_box_val(self) -> Self::ValType { *self }
}

trait TraitExtendsTraitTakesBoxVal: TraitTakesBoxVal {}

impl<T: TraitTakesBoxVal> TraitExtendsTraitTakesBoxVal for T {}

impl MyStruct {
    fn inherent_takes_box_val(self: Box<Self>) {}
}

fn takes_box_val(_: Box<MyStruct>) {}

pub fn foo() {
    let boxed = Box::new(MyStruct);
    let ref_type = boxed.trait_takes_box_ref();
    boxed.inherent_takes_box_ref();
    takes_box_ref(&boxed);

    let boxed = Box::new(MyStruct);
    let cloned_boxed = boxed.clone();
    let val_type = cloned_boxed.trait_takes_box_val();
    let cloned_boxed_2 = boxed.clone();
    cloned_boxed_2.inherent_takes_box_val();
    takes_box_val(boxed);

    <Box<MyStruct> as TraitTakesBoxRef>::trait_takes_box_ref(&Box::new(MyStruct));
    MyStruct::inherent_takes_box_ref(&Box::new(MyStruct));
}

fn main() {}