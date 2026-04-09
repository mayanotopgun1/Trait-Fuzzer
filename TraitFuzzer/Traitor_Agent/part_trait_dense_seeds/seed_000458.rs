#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(return_position_impl_trait_in_trait)]

#[derive(Debug)]
struct A<'a, 'b> where 'a : 'b { x: &'a isize, y: &'b isize }

trait ConstructA<'a, 'b> {
    fn new(x: &'a isize, y: &'b isize) -> Self;
}

impl<'a, 'b> ConstructA<'a, 'b> for A<'a, 'b> {
    fn new(x: &'a isize, y: &'b isize) -> Self {
        A { x, y }
    }
}

trait ConstructAPair<'a, 'b>: ConstructA<'a, 'b> where Self: Sized + core::fmt::Debug {
    fn new_pair(x1: &'a isize, y1: &'b isize, x2: &'a isize, y2: &'b isize) -> impl core::fmt::Debug {
        (Self::new(x1, y1), Self::new(x2, y2))
    }
}

impl<'a, 'b, T> ConstructAPair<'a, 'b> for T where T: ConstructA<'a, 'b> + core::fmt::Debug {}

fn main() {
    let x = 1;
    let y = 1;
    let pair = A::new_pair(&x, &y, &x, &y);
}