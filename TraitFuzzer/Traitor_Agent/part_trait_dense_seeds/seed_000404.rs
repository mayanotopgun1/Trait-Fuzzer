trait IterVec<T> { fn iter_vec<F>(&self, f: F) where F: FnMut(&T); }

impl<T, U> IterVec<U> for T where T: AsRef<[U]> {
    fn iter_vec<F>(&self, mut f: F) where F: FnMut(&U) {
        for x in self.as_ref() {
            f(x);
        }
    }
}

pub fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    let mut odds = 0;
    v.iter_vec(|i| {
        if *i % 2 == 1 {
            odds += 1;
        }
    });
    println!("{}", odds);
    assert_eq!(odds, 4);
}