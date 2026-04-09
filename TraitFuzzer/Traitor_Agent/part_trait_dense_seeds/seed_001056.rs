use std::panic;

struct Lies(usize);

impl Iterator for Lies {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.0 == 0 {
            None
        } else {
            self.0 -= 1;
            Some(self.0)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(2))
    }
}

trait CustomIterator: Iterator {
    fn custom_filter<F>(&mut self, mut predicate: F) -> usize
    where
        Self::Item: Copy,
        F: FnMut(Self::Item) -> bool,
    {
        let mut count = 0;
        while let Some(item) = self.next() {
            if predicate(item) {
                count += 1;
            }
        }
        count
    }
}

impl<T> CustomIterator for T where T: Iterator {}

fn main() {
    let r = panic::catch_unwind(|| {
        let mut lies_iter = Lies(10);
        let _ = lies_iter.custom_filter(|x| x > 3);
    });
    assert!(r.is_err());
}