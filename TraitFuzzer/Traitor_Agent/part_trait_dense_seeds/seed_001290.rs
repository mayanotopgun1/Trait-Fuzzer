trait FuncCall { fn call(self); }
impl<F> FuncCall for F where F: FnOnce() { fn call(self) { self(); } }

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