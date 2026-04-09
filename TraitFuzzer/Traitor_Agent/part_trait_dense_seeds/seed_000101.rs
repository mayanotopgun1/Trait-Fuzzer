#![feature(generic_associated_types)]

pub trait Trait1 {
    type C;
}

struct T1;
impl Trait1 for T1 {
    type C = usize;
}

pub trait Callback<T: Trait1>: FnMut(<T as Trait1>::C) {}
impl<T: Trait1, F: FnMut(<T as Trait1>::C)> Callback<T> for F {}

pub struct State<T: Trait1> {
    callback: Option<Box<dyn Callback<T>>>,
}
impl<T: Trait1> State<T> {
    fn new() -> Self {
        Self { callback: None }
    }

    fn set_callback(&mut self, cb: Box<dyn Callback<T>>) {
        self.callback = Some(cb);
    }

    fn test_cb<'a>(&'a mut self, d: <T as Trait1>::C) where T: Trait1 {
        (self.callback.as_mut().unwrap())(d)
    }
}

trait StateExt<T: Trait1> {
    fn new_ext() -> Self;
    fn set_callback_ext(&mut self, cb: Box<dyn Callback<T>>);
    fn test_cb_ext<'a>(&'a mut self, d: <T as Trait1>::C) where T: Trait1;
}
impl<T: Trait1> StateExt<T> for State<T> {
    fn new_ext() -> Self {
        Self::new()
    }

    fn set_callback_ext(&mut self, cb: Box<dyn Callback<T>>) {
        self.set_callback(cb);
    }

    fn test_cb_ext<'a>(&'a mut self, d: <T as Trait1>::C) where T: Trait1 {
        self.test_cb(d);
    }
}

fn main() {
    let mut s = State::<T1>::new_ext();
    s.set_callback_ext(Box::new(|x| println!("Callback called with: {}", x)));
    s.test_cb_ext(1);
}