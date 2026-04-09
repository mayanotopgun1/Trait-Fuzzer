trait Compare {
    fn compare(&self, other: &Self) -> bool;
}

impl<T> Compare for T
where
    T: PartialEq + PartialOrd,
{
    fn compare(&self, other: &Self) -> bool {
        self == other || self < other
    }
}

fn f<T: Compare>(_: T) {}

pub fn main() {
    f(3);
}