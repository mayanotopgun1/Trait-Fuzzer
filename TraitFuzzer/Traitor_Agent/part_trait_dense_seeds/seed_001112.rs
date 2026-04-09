#![allow(overflowing_literals)]

use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

static LOG: AtomicUsize = AtomicUsize::new(0);

struct D(u8);

trait DropLog {
    fn log_drop(&mut self);
}

impl Drop for D {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
        let old = LOG.load(Ordering::SeqCst);
        let _ = LOG.compare_exchange(
            old,
            old << 4 | self.0 as usize,
            Ordering::SeqCst,
            Ordering::SeqCst
        );
    }
}

impl DropLog for D {
    fn log_drop(&mut self) {}
}

fn main() {
    fn die() -> D { panic!("Oh no"); }
    let g = thread::spawn(|| {
        let mut b1: Box<[D; 4]> = Box::new([D(1), D(2), D(3), D(4)]);
        for d in b1.iter_mut() {
            d.log_drop();
        }

        let mut b2: Box<[D; 4]> = Box::new([D(5), D(6), D(7), D(8)]);
        for d in b2.iter_mut() {
            d.log_drop();
        }

        let mut b3: Box<[D; 4]> = Box::new([D(9), D(10), die(), D(12)]);
        for d in b3.iter_mut() {
            d.log_drop();
        }

        let mut b4: Box<[D; 4]> = Box::new([D(13), D(14), D(15), D(16)]);
        for d in b4.iter_mut() {
            d.log_drop();
        }
    });
    assert!(g.join().is_err());

    let expect = 0x__A_9__5_6_7_8__1_2_3_4;
    let actual = LOG.load(Ordering::SeqCst);
    assert!(actual == expect, "expect: 0x{:x} actual: 0x{:x}", expect, actual);
}