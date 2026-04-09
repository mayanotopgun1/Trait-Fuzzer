#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::{marker::PhantomData, ops::Mul};

pub enum Nil {}
pub struct Cons<T, L> {
    _phantom: PhantomData<(T, L)>,
}

pub trait Indices<const N: usize> {
    const RANK: usize;
    const NUM_ELEMS: usize;
}

impl<const N: usize> Indices<N> for Nil {
    const RANK: usize = 0;
    const NUM_ELEMS: usize = 1;
}

impl<T, I: Indices<N>, const N: usize> Indices<N> for Cons<T, I> {
    const RANK: usize = I::RANK + 1;
    const NUM_ELEMS: usize = I::NUM_ELEMS * N;
}

pub trait Concat<J> {
    type Output;
}

impl<J> Concat<J> for Nil {
    type Output = J;
}

impl<T, I, J> Concat<J> for Cons<T, I>
where
    I: Concat<J>,
{
    type Output = Cons<T, <I as Concat<J>>::Output>;
}

trait TensorTrait<I: Indices<N>, const N: usize>
where
    [u8; I::NUM_ELEMS]: Sized,
{
    fn new(data: [u8; I::NUM_ELEMS]) -> Self;
}

impl<I: Indices<N>, const N: usize> TensorTrait<I, N> for Tensor<I, N>
where
    [(); I::NUM_ELEMS]: ,
{
    fn new(data: [u8; I::NUM_ELEMS]) -> Self {
        Tensor {
            data,
            _phantom: PhantomData,
        }
    }
}

pub struct Tensor<I: Indices<N>, const N: usize>
where
    [(); I::NUM_ELEMS]: ,
{
    pub data: [u8; I::NUM_ELEMS],
    _phantom: PhantomData<()>,
}

impl<I: Indices<N>, J: Indices<N>, const N: usize> Mul<Tensor<J, N>> for Tensor<I, N>
where
    I: Concat<J>,
    <I as Concat<J>>::Output: Indices<N>,
    [(); I::NUM_ELEMS]: ,
    [(); J::NUM_ELEMS]: ,
    [(); <I as Concat<J>>::Output::NUM_ELEMS]: ,
{
    type Output = Tensor<<I as Concat<J>>::Output, N>;

    fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
        TensorTrait::new([0u8; <I as Concat<J>>::Output::NUM_ELEMS])
    }
}

fn main() {}