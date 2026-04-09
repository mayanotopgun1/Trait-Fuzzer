trait Doubler {
    fn double(&self, x: usize) -> usize;
}

impl Doubler for extern "C" fn(usize) -> usize {
    fn double(&self, x: usize) -> usize {
        self(x)
    }
}

impl<T> Doubler for T where T: Fn(usize) -> usize {
    fn double(&self, x: usize) -> usize {
        (self)(x * 2)
    }
}

pub fn main() {
    let f = |x: usize| x * 2;
    let x = (&f).double(22);
    assert_eq!(x, 44);
}