#![feature(type_alias_impl_trait, return_position_impl_trait_in_trait)]

trait Container {
    type Item;
    fn get(&self) -> impl core::fmt::Debug;
}

trait DebugExt: core::fmt::Debug {
    fn debug_custom(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomDebug({:?})", self)
    }
}

impl<T> DebugExt for T where T: core::fmt::Debug {}

struct T {
    f: Box<[Option<T>; 1]>
}

impl core::fmt::Debug for T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "T {{ f: {:?} }}", self.f)
    }
}

type Hidden = dyn core::fmt::Debug;

impl Container for T {
    type Item = Box<[Option<T>; 1]>;
    fn get(&self) -> impl core::fmt::Debug {
        &self.f
    }
}

fn main() {
    let x = T { f: Box::new([None]) };
    let _ = x.get();
}