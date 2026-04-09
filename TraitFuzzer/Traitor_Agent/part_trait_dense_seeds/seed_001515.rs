trait Marker {}

impl Marker for A {}
impl Marker for B {}
impl Marker for C {}

#[repr(Rust)]
struct A;

#[repr(Rust, align(16))]
struct B;

#[repr(Rust, packed)]
struct C;

fn main() {}