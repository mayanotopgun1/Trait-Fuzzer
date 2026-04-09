#![crate_type = "lib"]

use std::marker::PhantomData;

pub trait Sizable {
    const SIZE: usize;
}

impl<'a> Sizable for S<'a> {
    const SIZE: usize = 1;
}

pub struct S<'a> {
    pub m1: PhantomData<&'a u8>,
    pub m2: [u8; <S as Sizable>::SIZE],
}

impl<'a> S<'a>
{
    pub fn new() -> Self
    {
        Self
        {
            m1: PhantomData,
            m2: [0; <Self as Sizable>::SIZE],
        }
    }
}