#![feature(test)]
#![feature(cfg_eval)]

trait Testable {
    fn run(&self);
}

impl Testable for () {
    fn run(&self) {}
}

#[test]
fn f() {
    let test: () = ();
    test.run();
}

#[test]
fn f2() {
    let test: () = ();
    test.run();
}

trait InlineTestable {
    fn run_inline(&self);
}

impl InlineTestable for () {
    fn run_inline(&self) {}
}

#[test]
#[inline(always)]
fn f3() {
    let test: () = ();
    test.run_inline();
}

extern crate test;
use test::Bencher;

trait Benchable {
    fn bench(&self, b: &mut Bencher);
}

impl Benchable for () {
    fn bench(&self, _b: &mut Bencher) {}
}

#[bench]
fn f4(_: &mut Bencher) {
    let bench: () = ();
    bench.bench(_b);
}

#[cfg_eval]
struct S;

#[cfg_eval]
struct S2;

fn main() {}