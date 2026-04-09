#![allow(unused_must_use)]

use std::sync::mpsc::{channel, Sender};
use std::thread;

trait ChannelOps {
    fn send_value(&self, value: isize) -> Result<(), std::sync::mpsc::SendError<isize>>;
}

impl ChannelOps for Sender<isize> {
    fn send_value(&self, value: isize) -> Result<(), std::sync::mpsc::SendError<isize>> {
        self.send(value)
    }
}

pub fn main() {
    let (tx, rx) = channel();
    let t = thread::spawn(move || child(&tx));
    let y = rx.recv().unwrap();
    println!("received");
    println!("{}", y);
    assert_eq!(y, 10);
    t.join();
}

fn child(c: &Sender<isize>) {
    println!("sending");
    c.send_value(10).unwrap();
    println!("value sent");
}