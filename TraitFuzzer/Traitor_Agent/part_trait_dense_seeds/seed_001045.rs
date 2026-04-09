#![feature(type_alias_impl_trait, generic_const_exprs)]
#![allow(incomplete_features)]

pub trait BlockCipher {
    const BLOCK_SIZE: usize;
}

trait BlockSizeCheck<C: BlockCipher> {
    type CheckType;
    const CHECK_SIZE: bool = true;
}

impl<C: BlockCipher, const M: usize> BlockSizeCheck<C> for [u8; M]
where
    [u8; { M - C::BLOCK_SIZE }]: Sized,
{
    type CheckType = ();
}

struct FooCipher;
impl BlockCipher for FooCipher {
    const BLOCK_SIZE: usize = 64;
}

struct BarCipher;
impl BlockCipher for BarCipher {
    const BLOCK_SIZE: usize = 32;
}

pub struct Block<C>(#[allow(dead_code)] C);

pub fn test<C: BlockCipher, const M: usize>()
where
    [u8; { M - C::BLOCK_SIZE }]: Sized,
{
    let _check_type: <[u8; M] as BlockSizeCheck<C>>::CheckType = ();
}

fn main() {
    test::<FooCipher, 128>();
    test::<BarCipher, 64>();
}