const ARR: [usize; 5] = [5, 4, 3, 2, 1];

trait ArrayAccess {
    fn access(&self, index: usize) -> usize;
}

impl ArrayAccess for [usize; 5] {
    fn access(&self, index: usize) -> usize {
        self[index]
    }
}

fn main() {
    assert_eq!(3, ARR.access(ARR.access(3)));
}