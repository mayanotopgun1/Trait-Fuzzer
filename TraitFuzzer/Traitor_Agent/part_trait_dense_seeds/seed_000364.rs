#![feature(specialization)]

macro_rules! _foo {
    () => {};
}

trait MacroTrait {}
impl MacroTrait for () {}

trait DynMacroTrait {
    fn as_macro_trait(&self) -> &dyn MacroTrait;
}

default impl<T> DynMacroTrait for T where T: MacroTrait {
    default fn as_macro_trait(&self) -> &dyn MacroTrait {
        self
    }
}

// Explicit specialization for the unit type `()`
impl DynMacroTrait for () {
    fn as_macro_trait(&self) -> &dyn MacroTrait {
        self
    }
}

fn main() {
    let x = ();
    let _: &dyn MacroTrait = x.as_macro_trait();
}