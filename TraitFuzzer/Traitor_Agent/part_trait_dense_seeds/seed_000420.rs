#![deny(unused_allocation)]

#[derive(Clone)]
struct MyStruct;

trait TraitTakesBoxRef {
    fn trait_takes_box_ref(&self);
}

impl TraitTakesBoxRef for Box<MyStruct> {
    fn trait_takes_box_ref(&self) {}
}

trait TraitExtendsTraitTakesBoxRef: TraitTakesBoxRef {}

impl<T: TraitTakesBoxRef> TraitExtendsTraitTakesBoxRef for T {}

impl MyStruct {
    fn inherent_takes_box_ref(self: &Box<Self>) {}
}

fn takes_box_ref(_: &Box<MyStruct>) {}

trait TraitTakesBoxVal {
    fn trait_takes_box_val(self);
}

impl TraitTakesBoxVal for Box<MyStruct> {
    fn trait_takes_box_val(self) {}
}

trait TraitExtendsTraitTakesBoxVal: TraitTakesBoxVal {}

impl<T: TraitTakesBoxVal> TraitExtendsTraitTakesBoxVal for T {}

impl MyStruct {
    fn inherent_takes_box_val(self: Box<Self>) {}
}

fn takes_box_val(_: Box<MyStruct>) {}

pub fn foo() {
    let boxed = Box::new(MyStruct);
    boxed.trait_takes_box_ref();
    boxed.inherent_takes_box_ref();
    takes_box_ref(&boxed);

    let boxed = Box::new(MyStruct);
    let cloned_boxed = boxed.clone();
    cloned_boxed.trait_takes_box_val();
    let cloned_boxed_2 = boxed.clone();
    cloned_boxed_2.inherent_takes_box_val();
    takes_box_val(boxed);

    <Box<MyStruct> as TraitTakesBoxRef>::trait_takes_box_ref(&Box::new(MyStruct));
    MyStruct::inherent_takes_box_ref(&Box::new(MyStruct));
}

fn main() {}