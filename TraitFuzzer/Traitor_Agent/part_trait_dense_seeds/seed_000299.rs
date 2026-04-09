trait Sortable {
    fn sort(&mut self);
}

impl<T: Ord> Sortable for [T] {
    fn sort(&mut self) {
        self.sort_unstable();
    }
}

trait SortableExt: Sortable {}
impl<T: Sortable> SortableExt for T {}

fn main() {
    let n = 127;
    let mut objs: Vec<_> =
        (0..n).map(|i| [(i % 2) as u8; 125001]).collect();
    objs.sort();
}