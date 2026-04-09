trait Indexable {
    type Item;
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

impl<T> Indexable for [T] {
    type Item = T;
    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.get(index)
    }
}

trait IndexableExt: Indexable {}
impl<T> IndexableExt for T where T: Indexable {}

fn f<T>(it: &[T])
where
    [T]: Indexable<Item = T>,
{
    let _ = it.get(0);
}

fn main() {}