#![feature(impl_trait_in_assoc_type)]

trait PairwiseSub: DoubleEndedIterator<Item=isize> + Sized {
    fn pairwise_sub(self) -> impl Iterator<Item=isize>;
}

impl<T: DoubleEndedIterator<Item=isize> + Sized> PairwiseSub for T {
    fn pairwise_sub(mut self) -> impl Iterator<Item=isize> {
        std::iter::from_fn(move || {
            let front = self.next();
            let back = self.next_back();
            match (front, back) {
                (Some(f), Some(b)) => Some(b - f),
                _ => None,
            }
        })
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let r: isize = v.into_iter().pairwise_sub().sum();
    assert_eq!(r, 9);
}