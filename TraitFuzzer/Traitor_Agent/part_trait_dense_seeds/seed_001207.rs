#![feature(type_alias_impl_trait, box_patterns)]

trait PrintAndAssert {
    fn print_and_assert(&self, expected: i32);
}

impl PrintAndAssert for i32 {
    fn print_and_assert(&self, expected: i32) {
        println!("{}", self);
        assert_eq!(*self, expected);
    }
}

trait AdditionalTrait {
    fn print_and_assert_twice(&self, expected: i32);
}

impl<T: PrintAndAssert> AdditionalTrait for T {
    fn print_and_assert_twice(&self, expected: i32) {
        self.print_and_assert(expected);
        self.print_and_assert(expected);
    }
}

type Hidden = impl PrintAndAssert + AdditionalTrait;

#[define_opaque(Hidden)]
fn define_opaque() -> Hidden {
    100
}

pub fn main() {
    let boxed_value: Box<Hidden> = Box::new(define_opaque());
    match &*boxed_value {
        x => {
            x.print_and_assert_twice(100);
        }
    }
}