#![feature(specialization)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::rc::Rc;
use std::thread;

struct S(Rc<()>);

impl Clone for S {
    fn clone(&self) -> Self {
        if Rc::strong_count(&self.0) == 7 {
            panic!("oops");
        }

        S(self.0.clone())
    }
}

trait CloneExt: Clone {
    fn cloned_tuple(&self) -> (Self, Self, Self, Self);
    fn cloned_array(&self) -> [Self; 4];
}

default impl<T> CloneExt for T
where
    T: Clone,
{
    fn cloned_tuple(&self) -> (Self, Self, Self, Self) {
        (self.clone(), self.clone(), self.clone(), self.clone())
    }

    fn cloned_array(&self) -> [Self; 4] {
        [self.clone(), self.clone(), self.clone(), self.clone()]
    }
}

impl CloneExt for S {
    fn cloned_tuple(&self) -> (Self, Self, Self, Self) {
        if Rc::strong_count(&self.0) == 6 {
            panic!("specialized oops");
        }

        (self.clone(), self.clone(), self.clone(), self.clone())
    }

    fn cloned_array(&self) -> [Self; 4] {
        if Rc::strong_count(&self.0) == 6 {
            panic!("specialized oops");
        }

        [self.clone(), self.clone(), self.clone(), self.clone()]
    }
}

fn main() {
    let counter = Rc::new(());

    let ccounter = counter.clone();
    let result = std::panic::catch_unwind(move || {
        let _ = S(ccounter).cloned_tuple();
    });

    assert!(result.is_err());
    assert_eq!(1, Rc::strong_count(&counter));

    let ccounter = counter.clone();
    let child = std::panic::catch_unwind(move || {
        let _ = S(ccounter).cloned_array();
    });

    assert!(child.is_err());
    assert_eq!(1, Rc::strong_count(&counter));
}