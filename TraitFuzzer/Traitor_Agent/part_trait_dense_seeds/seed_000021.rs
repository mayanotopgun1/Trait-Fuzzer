mod m {
    pub struct S(u8);

    trait SZ {
        fn value(&self) -> u8;
    }

    impl SZ for S {
        fn value(&self) -> u8 {
            self.0
        }
    }

    use S as Z;
}

use m::*;

fn main() {}