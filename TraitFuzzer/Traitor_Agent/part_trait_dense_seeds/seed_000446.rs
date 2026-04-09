trait Comparator {
    fn compare(&self, y: &str) -> bool;
}

impl Comparator for &str {
    fn compare(&self, y: &str) -> bool {
        match *self {
            "foo" => y == "foo",
            _ => y == "bar",
        }
    }
}

pub fn main() {
    assert!("foo".compare("foo"));
}