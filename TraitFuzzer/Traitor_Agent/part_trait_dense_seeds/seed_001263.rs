#![expect(incomplete_features)]
#![feature(explicit_tail_calls)]

trait F { fn f(&mut self); }
impl F for () { fn f(&mut self) { let _y = String::new(); become self.f(); } }

fn main() {
    let mut x = ();
    x.f();
}