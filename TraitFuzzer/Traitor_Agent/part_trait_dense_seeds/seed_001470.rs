trait SizeOfVal {
    fn size_of_val(&self) -> usize;
}

impl<T> SizeOfVal for T {
    fn size_of_val(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

fn main() {
    let x = 1;
    println!("{}", x.size_of_val());
}