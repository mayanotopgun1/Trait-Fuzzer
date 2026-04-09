trait EmptyTrait {}

impl EmptyTrait for () {}

fn main() {
    #[cfg(crossbeam_loom)]
    let _ = ();
}