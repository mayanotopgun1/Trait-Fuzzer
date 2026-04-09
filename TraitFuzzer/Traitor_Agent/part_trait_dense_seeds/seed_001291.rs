#![feature(generic_associated_types)]

trait FuncCall {
    type CallType;
    fn call(self) -> Self::CallType;
}

impl<F> FuncCall for F
where
    F: FnOnce(),
{
    type CallType = ();
    fn call(self) -> Self::CallType {
        self();
    }
}

fn f<F>(p: F)
where
    F: FuncCall,
{
    p.call();
}

pub fn main() {
    let p = || ();
    f(p);
}