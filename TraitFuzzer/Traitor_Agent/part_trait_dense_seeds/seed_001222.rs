trait HelloWorld {
    fn greet(&self);
}

impl HelloWorld for () {
    fn greet(&self) {
        println!("Hello, World!");
    }
}

fn main() {
    let _ = <() as HelloWorld>::greet(&());
}