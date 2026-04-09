trait Each<T> {
    fn each<F>(&self, f: F)
    where
        F: FnMut(&T);
}

impl<T> Each<T> for [T] {
    fn each<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        for val in self {
            f(val)
        }
    }
}

fn main() {
    let mut sum = 0_usize;
    let elems = [1_usize, 2, 3, 4, 5];
    elems.each(|val| sum += *val);
    assert_eq!(sum, 15);
}