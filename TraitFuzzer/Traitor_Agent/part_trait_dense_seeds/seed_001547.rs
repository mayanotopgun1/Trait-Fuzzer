#![crate_type = "lib"]
#![feature(specialization)]

trait Linkable {
    fn link(&self);
}

trait LinkableExt: Linkable {
    fn link_ext(&self) where Self: Sized { self.link() }
}

impl<S> LinkableExt for S where S: Linkable {}

default impl<T> Linkable for T {
    fn link(&self) {}
}

impl Linkable for () {
    fn link(&self) {}
}

#[link(name = "foo")]
extern "C" {}

fn main() {
    let _ = <() as LinkableExt>::link_ext(&());
}