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
    fn flatten(self) -> Flatten<Self>;
}

impl<I> FlattenTrait<I> for I
where
    I: Iterator<Item: IntoIterator>,
{
    fn flatten(mut self) -> Flatten<Self> {
        Flatten {
            inner: self.next().unwrap_or_else(|| unreachable!()).into_iter(),
        }
    }
}

fn main() {}