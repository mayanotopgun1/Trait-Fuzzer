#![feature(type_alias_impl_trait)]

type Opaque<'lt> = impl Sized + 'lt;

trait OpaqueTrait<'lt>: Iterator<Item = Opaque<'lt>> {}

impl<'lt, T: Iterator<Item = Opaque<'lt>>> OpaqueTrait<'lt> for T {}

#[define_opaque(Opaque)]
fn test<'a>(
    arg: impl Iterator<Item = &'a u8>,
) -> impl OpaqueTrait<'a> {
    arg
}

fn main() {}