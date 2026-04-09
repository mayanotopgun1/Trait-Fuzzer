trait Identity { fn id(self) -> Self; }
impl Identity for () { fn id(self) -> Self { self } }

fn f(u: ()) -> () {
    u.id()
}

pub fn main() {
    let u1: () = ();
    let mut _u2: () = f(u1);
    _u2 = ().id();
    ()
}