trait Destructure { fn destructure(&self) -> isize; }

impl Destructure for Option<isize> {
    fn destructure(&self) -> isize {
        match self {
            None => 0,
            Some(ref v) => *v,
        }
    }
}

pub fn main() {
    assert_eq!(Some(22).destructure(), 22);
}