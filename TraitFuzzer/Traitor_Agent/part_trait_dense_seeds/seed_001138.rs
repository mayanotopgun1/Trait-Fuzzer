use std::cell::RefCell;

struct S<'a>(i32, &'a RefCell<Vec<i32>>);

trait DropExt<'a> {
    fn drop_value(&mut self);
}

impl<'a> DropExt<'a> for S<'a> {
    fn drop_value(&mut self) {
        self.1.borrow_mut().push(self.0);
    }
}

impl<'a> Drop for S<'a> {
    fn drop(&mut self) {
        self.drop_value();
    }
}

fn test(drops: &RefCell<Vec<i32>>) {
    let mut foo = None;
    match foo {
        None => (),
        _ => return,
    }

    *(&mut foo) = Some((S(0, drops), S(1, drops)));

    match foo {
        Some((_x, _)) => {}
        _ => {}
    }
}

fn main() {
    let drops = RefCell::new(Vec::new());
    test(&drops);
    assert_eq!(*drops.borrow(), &[0, 1]);
}