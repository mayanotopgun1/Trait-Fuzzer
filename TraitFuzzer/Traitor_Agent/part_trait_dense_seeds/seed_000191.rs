#![feature(generic_associated_types)]
#![warn(unused_imports)]

#[macro_export]
macro_rules! mac { () => {} }

trait MacroUsage {
    type Output<'a> where Self: 'a;
    fn apply_macro(&self) -> Self::Output<'_>;
}

trait MacroUsageExt: MacroUsage {
    fn apply_macro_twice(&self) -> Self::Output<'_> {
        self.apply_macro()
    }
}

impl<T> MacroUsageExt for T where T: MacroUsage {}

impl MacroUsage for () {
    type Output<'a> = &'a ();
    fn apply_macro(&self) -> Self::Output<'_> {
        mac!();
        self
    }
}

fn main() {
    let _ = <() as MacroUsageExt>::apply_macro_twice(&());
}