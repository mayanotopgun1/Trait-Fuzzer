#![feature(specialization)]

trait MainTrait {
    fn main();
}

default impl<T> MainTrait for T {
    default fn main() {}
}

impl MainTrait for () {
    fn main() {}
}

fn main() {
    <() as MainTrait>::main();
}