fn main() {
    let _s = construct().borrow().consume_borrowed();
}

fn construct() -> Value { Value }

pub struct Value;

trait Borrowable<'a> {
    type Output;
    fn borrow(&'a self) -> Self::Output;
}

impl<'a> Borrowable<'a> for Value {
    type Output = Borrowed<'a>;
    fn borrow(&'a self) -> Borrowed<'a> { unimplemented!() }
}

pub struct Borrowed<'a> {
    _inner: Guard<'a, Value>,
}

trait Consumable {
    fn consume_borrowed(self) -> String;
}

impl<'a> Consumable for Borrowed<'a> {
    fn consume_borrowed(self) -> String { unimplemented!() }
}

pub struct Guard<'a, T: ?Sized + 'a> {
    _lock: &'a T,
}

impl<'a, T: ?Sized> Drop for Guard<'a, T> { fn drop(&mut self) {} }