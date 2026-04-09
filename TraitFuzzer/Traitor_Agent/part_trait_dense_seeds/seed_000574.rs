pub struct Item {
    _inner: &'static str,
}

pub struct Bar<T> {
    items: Vec<Item>,
    inner: T,
}

pub trait IntoBar<T> {
    fn into_bar(self) -> Bar<T>;
}

impl<'a, T> IntoBar<T> for &'a str where &'a str: Into<T> {
    fn into_bar(self) -> Bar<T> {
        Bar {
            items: Vec::new(),
            inner: self.into(),
        }
    }
}

pub trait ItemCheck {
    fn has_items(&self) -> bool;
}

impl<T> ItemCheck for Bar<T> {
    fn has_items(&self) -> bool {
        !self.items.is_empty()
    }
}

fn main() {}