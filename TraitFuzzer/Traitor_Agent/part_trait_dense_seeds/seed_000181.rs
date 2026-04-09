#![allow(dead_code)]

trait MatrixRow {
    fn dummy(&self) {}
    fn new_dummy(self: &Self) where Self: Sized { self.dummy(); }

    fn create_row<'a>(self: &'a Self) -> impl Iterator<Item = ()> + 'a;
}

struct Mat;

impl<'a> MatrixRow for &'a Mat {
    fn create_row<'b>(self: &'b Self) -> impl Iterator<Item = ()> + 'b {
        Rows { mat: *self }
    }
}

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