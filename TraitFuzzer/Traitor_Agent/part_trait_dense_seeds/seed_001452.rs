#![feature(core_intrinsics)]
#![feature(const_type_name)]
#![feature(const_trait_impl)]
#![allow(dead_code)]

const fn type_name_wrapper<T>(_: &T) -> &'static str {
    const { core::intrinsics::type_name::<T>() }
}

const trait Nameable {
    fn get_type_name(&self) -> &'static str;
}

impl<TA, TB, TC> const Nameable for Struct<TA, TB, TC> {
    fn get_type_name(&self) -> &'static str {
        type_name_wrapper(self)
    }
}

struct Struct<TA, TB, TC> {
    a: TA,
    b: TB,
    c: TC,
}

type StructInstantiation = Struct<i8, f64, bool>;

const CONST_STRUCT: StructInstantiation = StructInstantiation {
    a: 12,
    b: 13.7,
    c: false,
};

const CONST_STRUCT_NAME: &'static str = CONST_STRUCT.get_type_name();

fn main() {
    let non_const_struct = StructInstantiation {
        a: 87,
        b: 65.99,
        c: true,
    };

    let non_const_struct_name = non_const_struct.get_type_name();

    assert_eq!(CONST_STRUCT_NAME, non_const_struct_name);
}