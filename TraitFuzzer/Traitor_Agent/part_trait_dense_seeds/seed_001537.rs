#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

trait ClosureBox<'a> {
    type Closure: FnMut() + 'a;
    fn new(x: Self::Closure) -> Self;
    fn execute(&mut self);
}

impl<'a> ClosureBox<'a> for closure_box<'a> {
    type Closure = Box<dyn FnMut() + 'a>;

    fn new(x: Self::Closure) -> Self {
        closure_box { cl: x }
    }

    fn execute(&mut self) {
        (self.cl)();
    }
}

trait Executable<'a>: ClosureBox<'a> {
    fn run_once(&mut self);
}

impl<'a, T> Executable<'a> for T
where
    T: ClosureBox<'a>,
{
    fn run_once(&mut self) {
        self.execute();
    }
}

struct closure_box<'a> {
    cl: Box<dyn FnMut() + 'a>,
}

fn box_it<'a>(x: Box<dyn FnMut() + 'a>) -> closure_box<'a> {
    ClosureBox::new(x)
}

pub fn main() {
    let mut i = 3;
    assert_eq!(i, 3);
    {
        let cl = || i += 1;
        let mut cl_box = box_it(Box::new(cl));
        cl_box.run_once();
    }
    assert_eq!(i, 4);
}