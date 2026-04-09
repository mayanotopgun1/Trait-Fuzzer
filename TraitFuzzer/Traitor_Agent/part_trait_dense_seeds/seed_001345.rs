trait ArrayEq<T> {
    fn eq(&self, other: &[T]) -> bool;
}

impl ArrayEq<isize> for [isize; 4] {
    fn eq(&self, other: &[isize]) -> bool {
        self.iter().zip(other).all(|(a, b)| a == b)
    }
}

static FOO: [isize; 4] = [32; 4];
static BAR: [isize; 4] = [32, 32, 32, 32];

fn check_array_eq<T>(arr: &[T], other: &[T]) -> bool
where
    T: 'static + std::cmp::PartialEq,
{
    arr.iter().zip(other).all(|(a, b)| a == b)
}

pub fn main() {
    assert_eq!(check_array_eq(&FOO, &BAR), true);
}