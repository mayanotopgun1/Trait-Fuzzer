#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]

trait Take {
    type Out;
    fn take(self) -> Self::Out;
}

impl Take for isize {
    type Out = isize;
    fn take(self) -> Self::Out { self }
}

trait TakeView: Take + Copy {
    fn take_ref(&self) -> <Self as Take>::Out where <Self as Take>::Out: Copy { self.take() }
}

impl<T: Take + Copy> TakeView for T {}

fn the_loop<T: TakeView>(take_fn: T) where <T as Take>::Out: Copy {
    let mut list = Vec::new();
    loop {
        let x = 5;
        if x > 3 {
            list.push(take_fn.take_ref());
        } else {
            break;
        }
    }
}

pub fn main() {}