trait Adder {
    fn add(&mut self, x: isize) -> isize;
}

impl<F> Adder for F
where
    F: FnMut(isize) -> isize,
{
    fn add(&mut self, x: isize) -> isize {
        self(x)
    }
}

fn make_adder(x: isize) -> Box<dyn Adder + 'static> {
    Box::new(move |y| { x + y })
}

pub fn main() {
    let mut adder = make_adder(3);
    let z = adder.add(2);
    println!("{}", z);
    assert_eq!(z, 5);
}