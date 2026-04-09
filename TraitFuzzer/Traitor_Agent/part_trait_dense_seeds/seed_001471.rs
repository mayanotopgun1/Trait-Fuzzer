#![feature(specialization)]

trait SizeOfVal {
    fn size_of_val(&self) -> usize;
}

default impl<T> SizeOfVal for T {
    fn size_of_val(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

impl SizeOfVal for i32 {
    fn size_of_val(&self) -> usize {
        4 // Specialized implementation for i32
    }
}

fn main() {
    let x = 1;
    println!("{}", x.size_of_val());
}