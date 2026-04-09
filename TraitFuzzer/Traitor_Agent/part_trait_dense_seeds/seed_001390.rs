struct T {}

trait TraitT {
    fn method(&self);
}

impl TraitT for T {
    fn method(&self) {}
}

fn main() {
    let t = T {};
    t.method();
}