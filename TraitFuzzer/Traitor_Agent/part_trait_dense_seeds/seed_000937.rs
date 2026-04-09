macro_rules! third {
    ($e:expr) => ({let x = 2; $e[x]})
}

trait Indexable {
    type Output;
    fn index(&self, index: usize) -> Self::Output;
}

impl<T: Clone> Indexable for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> Self::Output {
        self[index].clone()
    }
}

fn main() {
    let x = vec![10_usize, 11_usize, 12_usize, 13_usize];
    let t = third!(x);
    assert_eq!(t, 12_usize);
}