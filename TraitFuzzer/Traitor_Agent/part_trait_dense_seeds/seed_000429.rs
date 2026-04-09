trait Compare {
    fn compare(&self, other: &Self) -> bool;
}

impl<T: PartialEq + PartialOrd> Compare for T {
    fn compare(&self, other: &Self) -> bool {
        self == other || self < other
    }
}

fn f<T: Compare>(_: T) {}

pub fn main() {
    f(3);
}