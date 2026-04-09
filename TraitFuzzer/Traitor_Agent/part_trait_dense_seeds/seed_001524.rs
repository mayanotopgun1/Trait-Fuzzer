trait PairwiseSub: DoubleEndedIterator<Item=isize> + Sized {
    fn pairwise_sub(mut self) -> isize {
        let mut result = 0;
        loop {
            let front = self.next();
            let back = self.next_back();
            match (front, back) {
                (Some(f), Some(b)) => { result += b - f; }
                _ => { return result; }
            }
        }
    }
}

impl<T: DoubleEndedIterator<Item=isize> + Sized> PairwiseSub for T {}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let r = v.into_iter().pairwise_sub();
    assert_eq!(r, 9);
}