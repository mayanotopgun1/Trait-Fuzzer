use std::ops::Index;

trait IndexWithFn {
    type Output;
    fn index_with_fn(&self, f: fn()) -> &Self::Output;
}

impl IndexWithFn for S {
    type Output = ();
    fn index_with_fn(&self, _: fn()) -> &() {
        &UNIT
    }
}

struct S;

impl Index<fn()> for S {
    type Output = ();
    fn index(&self, f: fn()) -> &() {
        self.index_with_fn(f)
    }
}

fn bar() {}
static UNIT: () = ();

fn main() {
    let s = S;
    s.index_with_fn(bar);
    s[bar];
}