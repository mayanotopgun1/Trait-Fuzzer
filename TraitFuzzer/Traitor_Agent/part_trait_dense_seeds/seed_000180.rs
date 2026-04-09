#![allow(dead_code)]

trait MatrixRow { 
    fn dummy(&self) {} 
    fn new_dummy(self: &Self) where Self: Sized { self.dummy(); } 
}

struct Mat;

impl<'a> MatrixRow for &'a Mat {}

struct Rows<M: MatrixRow> {
    mat: M,
}

impl<'a> Iterator for Rows<&'a Mat> {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        unimplemented!()
    }
}

fn main() {}