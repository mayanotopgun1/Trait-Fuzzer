trait ConstMir { fn const_mir() -> f32; }

impl ConstMir for () {
    fn const_mir() -> f32 { 9007199791611905.0 }
}

fn main() {
    let original = "9007199791611905.0";
    let expected = "9007200000000000";

    assert_eq!(<() as ConstMir>::const_mir().to_string(), expected);
    assert_eq!(original.parse::<f32>().unwrap().to_string(), expected);
}