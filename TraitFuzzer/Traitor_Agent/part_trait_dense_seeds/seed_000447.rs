trait ArrayInit {
    fn min_array_ok() -> [i128; 1];
    fn min_array_nok() -> [i128; 1];
}

impl ArrayInit for () {
    fn min_array_ok() -> [i128; 1] {
        [i128::MIN]
    }

    fn min_array_nok() -> [i128; 1] {
        [i128::MIN; 1]
    }
}

fn main() {
    assert_eq!(<()>::min_array_ok(), [-170141183460469231731687303715884105728i128]);
    assert_eq!(<()>::min_array_nok(), [-170141183460469231731687303715884105728i128]);
}