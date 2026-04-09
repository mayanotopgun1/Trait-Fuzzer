#![allow(internal_features)]
#![feature(rustc_attrs, never_type, return_position_impl_trait_in_trait)]
#![cfg_attr(unit, rustc_never_type_options(fallback = "unit"))]
#![cfg_attr(never, rustc_never_type_options(fallback = "never"))]

type Infallible = !;

#[derive(Debug)]
struct E;

impl From<Infallible> for E {
    fn from(_: Infallible) -> E {
        E
    }
}

trait TryFromU32 {
    type Error;
    fn try_from(x: u32) -> Result<u32, Self::Error>;
}

trait TryFromExt: TryFromU32 {
    fn safe_try_from(x: u32) -> Option<u32> {
        <Self as TryFromU32>::try_from(x).ok()
    }
}

impl<T: TryFromU32> TryFromExt for T {}

impl TryFromU32 for Infallible {
    type Error = Infallible;
    fn try_from(x: u32) -> Result<u32, Self::Error> {
        Ok(x)
    }
}

fn _f() -> impl std::fmt::Debug {
    <! as TryFromExt>::safe_try_from(1u32).ok_or(E)?;
    Ok::<(), E>(())
}

fn main() {}