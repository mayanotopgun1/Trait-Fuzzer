trait ArrayLike {
    type Item;
    fn len(&self) -> usize;
}

impl<T, const N: usize> ArrayLike for [T; N] {
    type Item = T;
    fn len(&self) -> usize {
        self.len()
    }
}

fn main() {
    let x: Box<dyn ArrayLike<Item = usize>> = Box::new([0usize; 0xffff_ffff]);
    let _ = x.len();
}