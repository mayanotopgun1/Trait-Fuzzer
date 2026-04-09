fn borrow<T>(x: &T) -> &T { x }

trait Borrowable {
    fn borrow(&self) -> &Self;
}

impl<T> Borrowable for T {
    fn borrow(&self) -> &Self {
        self
    }
}

pub fn main() {
    let x: Box<_> = Box::new(3);
    loop {
        let y = x.borrow();
        assert_eq!(*x, **y);
        break;
    }
}