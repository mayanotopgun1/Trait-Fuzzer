trait Greet {
    fn greet(&self);
}

impl Greet for () {
    fn greet(&self) {
        println!("Hello world!");
    }
}

fn main() {
    let unit = ();
    unit.greet();
}