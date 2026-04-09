#![feature(generic_associated_types)]

trait ArrayInit {
    type Out;
    fn min_array_ok() -> Self::Out;
    fn min_array_nok() -> Self::Out;
}

impl ArrayInit for () {
    type Out = [i128; 1];
    fn min_array_ok() -> Self::Out {
        [i128::MIN]
    }

    fn min_array_nok() -> Self::Out {
        [i128::MIN; 1]
    }
}

fn main() {
    assert_eq!(<()>::min_array_ok(), [-170141183460469231731687303715884105728i128]);
    assert_eq!(<()>::min_array_nok(), [-170141183460469231731687303715884105728i128]);
}