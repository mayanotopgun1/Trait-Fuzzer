#![allow(unused_mut)]
#![allow(unused_assignments)]

use std::sync::mpsc::{Sender, Receiver, channel};

trait MessageSender {
    fn send_message(&self, message: isize) -> Result<(), ()>;
}

impl MessageSender for Sender<isize> {
    fn send_message(&self, message: isize) -> Result<(), ()> {
        self.send(message).map_err(|_| ())
    }
}

trait MessageReceiver {
    fn receive_message(&self) -> Result<isize, ()>;
}

impl MessageReceiver for Receiver<isize> {
    fn receive_message(&self) -> Result<isize, ()> {
        self.recv().map_err(|_| ())
    }
}

pub fn main() {
    test00();
}

fn test00() {
    let mut r: isize = 0;
    let mut sum: isize = 0;
    let (tx, rx) = channel();
    let mut tx0 = tx.clone();
    let mut tx1 = tx.clone();
    let mut tx2 = tx.clone();
    let mut tx3 = tx.clone();
    let number_of_messages: isize = 1000;
    let mut i: isize = 0;

    while i < number_of_messages {
        tx0.send_message(i + 0).unwrap();
        tx1.send_message(i + 0).unwrap();
        tx2.send_message(i + 0).unwrap();
        tx3.send_message(i + 0).unwrap();
        i += 1;
    }

    i = 0;
    while i < number_of_messages {
        r = rx.receive_message().unwrap();
        sum += r;
        r = rx.receive_message().unwrap();
        sum += r;
        r = rx.receive_message().unwrap();
        sum += r;
        r = rx.receive_message().unwrap();
        sum += r;
        i += 1;
    }

    assert_eq!(sum, 1998000);
}