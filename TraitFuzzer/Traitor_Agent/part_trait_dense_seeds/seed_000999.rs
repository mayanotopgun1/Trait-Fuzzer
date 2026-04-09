#![feature(specialization)]

trait OneTrait { fn one(&self) -> usize; }

struct OneStruct;

impl OneTrait for OneStruct {
    fn one(&self) -> usize { 1 }
}

mod a {
    use super::OneTrait;

    pub trait TwoTrait: OneTrait {
        fn two(&self) -> usize;
    }

    impl<T: OneTrait> TwoTrait for T {
        default fn two(&self) -> usize {
            self.one() + self.one()
        }
    }
}

mod b {
    use super::{OneTrait, a::TwoTrait};

    pub trait ThreeTrait: OneTrait + TwoTrait {
        fn three(&self) -> usize;
    }

    impl<T: OneTrait + TwoTrait> ThreeTrait for T {
        default fn three(&self) -> usize {
            self.one() + self.two()
        }
    }
}

use a::TwoTrait;
use b::ThreeTrait;

fn main() {
    let one_instance = OneStruct;
    assert_eq!(one_instance.one(), 1);
    assert_eq!(one_instance.two(), 2);
    assert_eq!(one_instance.three(), 3);
}