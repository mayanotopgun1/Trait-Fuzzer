#![feature(type_alias_impl_trait)]
#![warn(rust_2021_prelude_collisions)]

trait TryIntoU32 {
    type Error;
    fn try_into(&self) -> Result<u32, Self::Error>;
}

impl TryIntoU32 for u8 {
    type Error = ();
    fn try_into(&self) -> Result<u32, Self::Error> {
        Ok(22)
    }
}

trait TryIntoU32Ext: TryIntoU32 {
    fn try_into_default(&self) -> u32 {
        TryIntoU32::try_into(self).unwrap_or_default()
    }
}

impl<T: ?Sized + TryIntoU32> TryIntoU32Ext for T {}

mod inner {
    use super::{TryIntoU32, get_dyn_trait, TryIntoU32Ext};

    pub struct TryIntoU32Impl;

    impl TryIntoU32 for TryIntoU32Impl {
        type Error = ();
        fn try_into(&self) -> Result<u32, Self::Error> {
            Ok(0)
        }
    }

    type DynTryIntoU32 = dyn for<'a> TryIntoU32<Error = ()>;

    pub fn test() -> u32 {
        get_dyn_trait().try_into_default()
    }
}

fn get_dyn_trait() -> Box<dyn for<'a> TryIntoU32<Error = ()>> {
    Box::new(inner::TryIntoU32Impl)
}

fn main() {
    dbg!(inner::test());
}