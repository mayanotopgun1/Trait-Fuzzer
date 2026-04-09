trait TestIterator {
    fn test(&mut self);
}

impl<T: Iterator<Item = i32>> TestIterator for T {
    fn test(&mut self) {
        for x in self {
            assert_eq!(x, 1)
        }
    }
}

fn main() {
    let v = vec![1];
    (&mut v.into_iter()).test();
}