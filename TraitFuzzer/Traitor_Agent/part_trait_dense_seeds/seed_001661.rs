trait LoopControl { fn control(&self); }
impl LoopControl for () { fn control(&self) {} }

fn main() {
    let _: () = ();
    loop { ().control(); }
}