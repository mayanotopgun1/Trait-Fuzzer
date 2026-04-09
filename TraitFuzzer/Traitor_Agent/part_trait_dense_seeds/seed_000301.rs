#![allow(dead_code)]

trait NestedOperations {
    fn method_push_local(&mut self);
}

struct HasNested {
    nest: Vec<Vec<isize>>,
}

impl NestedOperations for HasNested {
    fn method_push_local(&mut self) {
        self.nest[0].push(0);
    }
}

pub fn main() {}