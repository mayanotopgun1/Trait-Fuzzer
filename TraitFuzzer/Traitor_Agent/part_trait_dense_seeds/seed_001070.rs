use std::thread;

trait Runnable {
    fn run(self);
}

impl Runnable for isize {
    fn run(self) {
        println!("{}", self);
        assert_eq!(self, 10);
    }
}

pub fn main() {
    thread::spawn(move || 10.run()).join().ok().unwrap();
}