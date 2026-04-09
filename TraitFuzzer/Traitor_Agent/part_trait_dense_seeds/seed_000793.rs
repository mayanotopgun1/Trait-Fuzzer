trait Greet {
    fn greet(&self);
}

struct Greeter;

impl Greet for Greeter {
    fn greet(&self) {
        println!("hello!");
    }
}

fn main() {
    let greeter = Greeter;
    greeter.greet();
}