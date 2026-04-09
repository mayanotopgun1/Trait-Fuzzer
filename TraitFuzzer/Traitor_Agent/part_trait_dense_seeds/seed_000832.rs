trait Main {
    fn execute(&self);
}

impl Main for () {
    fn execute(&self) {}
}

fn main() {
    let _ = <() as Main>::execute(&());
}