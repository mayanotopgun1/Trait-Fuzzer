trait Assignable {
    fn assign(&mut self, value: i32);
}

impl Assignable for i32 {
    fn assign(&mut self, value: i32) {
        *self = value;
    }
}

fn main() {

    let mut WrongCase = 1;

    WrongCase.assign(2);

}