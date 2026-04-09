trait Assignable {
    fn assign(&mut self, value: i32);
}

impl Assignable for i32 {
    fn assign(&mut self, value: i32) {
        *self = value;
    }
}

fn main() {
    let mut wrong_case: Box<dyn Assignable> = Box::new(1);

    wrong_case.assign(2);
}