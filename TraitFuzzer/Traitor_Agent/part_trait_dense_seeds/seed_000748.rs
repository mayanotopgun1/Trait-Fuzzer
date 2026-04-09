pub use hello::*;

pub trait SayHello {
    fn say_hello(&self);
}

impl SayHello for () {
    fn say_hello(&self) {
        println!("hello");
    }
}

pub mod say {
    pub use crate::SayHello;

    pub fn hello() {
        let unit = ();
        unit.say_hello();
    }
}

pub trait Hello: SayHello {
    fn hello(&self);
}

impl Hello for () {
    fn hello(&self) {
        self.say_hello();
    }
}

pub mod hello {
    use crate::Hello;

    pub fn hello() {
        let unit = ();
        unit.hello();
    }
}

fn main() {
    hello();
}