#![feature(specialization)]

trait Mirror {
    type Assoc;
}
default impl<T> Mirror for T {
    type Assoc = u8; // Default associated type
}

struct MyStruct;

impl Mirror for MyStruct {
    type Assoc = String; // Specialized associated type
}

trait Foo {}
trait Bar {}

trait BarCheck: Bar {}
impl<T> BarCheck for T where T: Bar {}

fn main() {}