trait Buildable {
    fn build(&self) -> Vec<isize>;
}

impl Buildable for () {
    fn build(&self) -> Vec<isize> {
        panic!();
    }
}

struct Blk {
    node: Vec<isize>,
}

impl BuildableExt for Blk {
    fn new_builder() -> Self {
        let vec = <() as Buildable>::build(&());
        Blk { node: vec }
    }
}

trait BuildableExt {
    fn new_builder() -> Self
    where
        Self: Sized;
}

fn main() {
    let _blk = Blk::new_builder();
}