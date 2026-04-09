struct X {
    a: isize
}

trait Changer : Sized {
    fn change(mut self) -> Self {
        self.set_to(55);
        self
    }

    fn change_again(mut self: Box<Self>) -> Box<Self> {
        self.set_to(45);
        self
    }

    fn set_to(&mut self, a: isize);
}

impl Changer for X {
    fn set_to(&mut self, a: isize) {
        self.a = a;
    }
}

trait ChangeExt: Changer {
    fn change_with_value(mut self, value: isize) -> Self {
        self.set_to(value);
        self
    }

    fn boxed_change_with_value(self: Box<Self>, value: isize) -> Box<Self> {
        let mut this = *self; // Dereference and copy to make it mutable
        this.set_to(value);
        Box::new(this)
    }
}

impl<T: Changer> ChangeExt for T {}

pub fn main() {
    let x = X { a: 32 };
    let new_x = x.change();
    assert_eq!(new_x.a, 55);

    let x: Box<_> = Box::new(new_x);
    let new_x = x.change_again();
    assert_eq!(new_x.a, 45);

    let x = X { a: 32 };
    let new_x = x.change_with_value(100);
    assert_eq!(new_x.a, 100);

    let x: Box<_> = Box::new(X { a: 32 });
    let new_x = x.boxed_change_with_value(200);
    assert_eq!(new_x.a, 200);
}