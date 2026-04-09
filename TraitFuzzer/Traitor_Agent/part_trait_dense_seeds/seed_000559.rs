#![feature(impl_trait_in_assoc_type)]

pub trait IntoDoubleEndedIterator<'a> {
    type Item;
    fn into_double_ended_iterator(self) -> Box<dyn DoubleEndedIterator<Item = Self::Item> + 'a>;
}

impl<'a> IntoDoubleEndedIterator<'a> for Vec<(u32, &'a u32)> {
    type Item = (u32, &'a u32);
    fn into_double_ended_iterator(self) -> Box<dyn DoubleEndedIterator<Item = Self::Item> + 'a> {
        Box::new(self.into_iter())
    }
}

pub fn iter<'a>(v: Vec<(u32, &'a u32)>) -> impl DoubleEndedIterator<Item = (u32, &u32)> + 'a {
    v.into_double_ended_iterator()
}

fn main() {}