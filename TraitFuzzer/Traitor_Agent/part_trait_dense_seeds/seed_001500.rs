#![feature(generic_associated_types)]

use thing::*;

#[derive(Debug)]
pub enum Thing {
    Foo
}

trait ThingTrait<'a> {
    type Out;
    fn new() -> Self::Out;
}

impl<'a> ThingTrait<'a> for Thing {
    type Out = Thing;
    fn new() -> Self::Out {
        Thing::Foo
    }
}

mod tests {
    use super::*;

    trait TestTrait {
        fn test_thing(&self);
    }

    impl TestTrait for () {
        fn test_thing(&self) {
            let thing: crate::Thing = <crate::Thing as crate::ThingTrait>::new();
        }
    }

    fn main() {
        let _: () = ().test_thing();
    }
}

mod thing {
    pub enum Thing {
        Bar
    }

    trait ThingTrait<'a> {
        type Out;
        fn new() -> Self::Out;
    }

    impl<'a> ThingTrait<'a> for Thing {
        type Out = Thing;
        fn new() -> Self::Out {
            Thing::Bar
        }
    }
}

fn main() { }