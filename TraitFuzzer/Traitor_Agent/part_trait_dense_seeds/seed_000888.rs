#[derive(Copy, Clone)]
struct X { vec: &'static [isize] }

static V: &'static [X] = &[X { vec: &[1, 2, 3] }];

trait VecAccess {
    fn get_vec(&self) -> &'static [isize];
}

impl VecAccess for X {
    fn get_vec(&self) -> &'static [isize] {
        self.vec
    }
}

pub fn main() {
    for &v in V {
        println!("{:?}", v.get_vec());
    }
}