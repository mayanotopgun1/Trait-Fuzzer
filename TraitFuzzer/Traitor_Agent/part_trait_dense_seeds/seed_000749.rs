#![feature(return_position_impl_trait_in_trait)]

pub use hello::*;

pub trait SayHello {
    fn say_hello(&self) -> impl core::fmt::Debug;
}

impl SayHello for () {
    fn say_hello(&self) -> impl core::fmt::Debug {
        println!("hello");
        "hello"
    }
}

pub mod say {
    pub use crate::SayHello;

    pub fn hello() {
        let unit = ();
        let _ = unit.say_hello();
    }
}

pub trait Hello: SayHello {
    fn hello(&self) -> impl core::fmt::Debug;
}

impl Hello for () {
    fn hello(&self) -> impl core::fmt::Debug {
        self.say_hello()
    }
}

pub mod hello {
    use crate::Hello;

    pub fn hello() {
        let unit = ();
        let _ = unit.hello();
    }
}

fn main() {
    hello();
}