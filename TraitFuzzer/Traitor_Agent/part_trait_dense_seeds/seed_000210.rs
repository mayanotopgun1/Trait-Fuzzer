#![feature(generic_associated_types)]

pub struct Flatten<I>
where
    I: Iterator<Item: IntoIterator>,
{
    inner: <I::Item as IntoIterator>::IntoIter,
}

trait FlattenTrait<I>: Iterator<Item: IntoIterator>
where
    Self: Sized,
    Self::Item: IntoIterator,
{
    type Flattened<'a> where I: 'a;

    fn flatten(self) -> Self::Flattened<'static>;
}

impl<I> FlattenTrait<I> for I
where
    I: Iterator<Item: IntoIterator> + 'static,
{
    type Flattened<'a> = Flatten<I> where I: 'a;

    fn flatten(mut self) -> Self::Flattened<'static> {
        Flatten {
            inner: self.next().unwrap_or_else(|| unreachable!()).into_iter(),
        }
    }
}

fn main() {}