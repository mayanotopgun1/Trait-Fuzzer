trait Sortable {
    type Item;
    fn sort_by_key(&mut self, f: Box<dyn FnMut(&Self::Item) -> usize>);
}

impl<T> Sortable for Vec<T> {
    type Item = T;
    fn sort_by_key(&mut self, f: Box<dyn FnMut(&Self::Item) -> usize>) {
        self.sort_by_key(f)
    }
}

fn main() {
    let mut v: Vec<&()> = Vec::new();
    v.sort_by_key(Box::new(|&r| r as *const () as usize));
}