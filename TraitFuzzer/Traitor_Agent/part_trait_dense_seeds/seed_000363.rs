#[track_caller]
macro_rules! _foo {
    () => {};
}

trait MacroTrait {}
impl MacroTrait for () {}

trait DynMacroTrait {
    fn as_macro_trait(&self) -> &dyn MacroTrait;
}

impl<T: MacroTrait> DynMacroTrait for T {
    fn as_macro_trait(&self) -> &dyn MacroTrait {
        self
    }
}

fn main() {
    let x = ();
    let _: &dyn MacroTrait = x.as_macro_trait();
}