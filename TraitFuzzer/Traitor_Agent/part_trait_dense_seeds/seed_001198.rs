#![allow(overflowing_literals)]

use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

static LOG: AtomicUsize = AtomicUsize::new(0);

trait Loggable {
    fn log(&self);
}

impl Loggable for u8 {
    fn log(&self) {
        println!("Logging {}", self);
        let old = LOG.load(Ordering::SeqCst);
        let _ = LOG.compare_exchange(
            old,
            (old << 4) | (*self as usize),
            Ordering::SeqCst,
            Ordering::SeqCst
        );
    }
}

struct D(u8);

impl Drop for D {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
        self.0.log();
    }
}

fn main() {
    fn die() -> D { panic!("Oh no"); }

    let g = thread::spawn(|| {
        let _b1: Box<[D; 4]> = Box::new([D(1), D(2), D(3), D(4)]);
        let _b2: Box<[D; 4]> = Box::new([D(5), D(6), D(7), D(8)]);
        let _b3: Box<[D; 4]> = Box::new([D(9), D(10), die(), D(12)]);
        let _b4: Box<[D; 4]> = Box::new([D(13), D(14), D(15), D(16)]);
    });

    assert!(g.join().is_err());

    let expect = 0xA_9_5_6_7_8_1_2_3_4;
    let actual = LOG.load(Ordering::SeqCst);
    assert!(actual == expect, "expect: 0x{:x} actual: 0x{:x}", expect, actual);
}