#![feature(specialization)]

trait MainTrait {
    fn execute(&self);
}

default impl<T> MainTrait for T {
    fn execute(&self) {
        println!("Generic execution");
    }
}

impl MainTrait for () {
    fn execute(&self) {
        println!("Unit execution");
    }
}

fn main() {
    let _: () = ().execute();
}