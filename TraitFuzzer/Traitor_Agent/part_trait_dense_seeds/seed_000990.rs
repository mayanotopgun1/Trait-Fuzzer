trait Main { fn main(&self); }
struct M;
impl Main for M { fn main(&self) {} }
fn main() { let m = M; m.main(); }