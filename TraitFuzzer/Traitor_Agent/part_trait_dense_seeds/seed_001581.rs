pub trait ElementLike {}

trait LocatedTrait<T> where T: ElementLike {
    fn get_inner(&self) -> &T;
}

pub struct Located<T> where T: ElementLike {
    inner: T,
}

impl<T> LocatedTrait<T> for Located<T> where T: ElementLike {
    fn get_inner(&self) -> &T {
        &self.inner
    }
}

pub struct BlockElement<'a>(&'a str);

impl ElementLike for BlockElement<'_> {}


pub struct Page<'a> {

    pub elements: Vec<Located<BlockElement<'a>>>,
}

use std::ops::Index; // Import the Index trait to use its methods

trait PageTrait<'a>: std::ops::Index<usize, Output = Located<BlockElement<'a>>> {
    fn get_element(&self, idx: usize) -> &Located<BlockElement<'a>>;
}

impl<'a> PageTrait<'a> for Page<'a> {
    fn get_element(&self, idx: usize) -> &Located<BlockElement<'a>> {
        self.index(idx)
    }
}

impl<'a, __IdxT> std::ops::Index<__IdxT> for Page<'a> where
    Vec<Located<BlockElement<'a>>>: std::ops::Index<__IdxT>
{
    type Output =
        <Vec<Located<BlockElement<'a>>> as
        std::ops::Index<__IdxT>>::Output;

    #[inline]
    fn index(&self, idx: __IdxT) -> &Self::Output {
        <Vec<Located<BlockElement<'a>>> as
                std::ops::Index<__IdxT>>::index(&self.elements, idx)
    }
}

fn main() {}