use std::ops::{Deref, DerefMut};

trait BoxFieldAccess {
    fn access_fields(&mut self);
}

impl BoxFieldAccess for Box<(i32, i32)> {
    fn access_fields(&mut self) {
        let _a = &mut self.0;
        let _b = &mut self.1;
    }
}

trait BoxDestructure {
    fn destructure(&mut self);
}

impl BoxDestructure for Box<(i32, i32)> {
    fn destructure(&mut self) {
        let (ref mut _head, ref mut _tail) = **self;
    }
}

struct Wrap<T>(T);

impl<T> Deref for Wrap<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Wrap<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

trait SmartFieldAccess {
    fn access_fields(&mut self);
}

impl SmartFieldAccess for Wrap<(i32, i32)> {
    fn access_fields(&mut self) {
        let _a = &mut self.0;
        let _b = &mut self.1;
    }
}

trait SmartDestructure {
    fn destructure(&mut self);
}

impl SmartDestructure for Wrap<(i32, i32)> {
    fn destructure(&mut self) {
        let (ref mut _head, ref mut _tail) = **self;
    }
}

fn main() {}