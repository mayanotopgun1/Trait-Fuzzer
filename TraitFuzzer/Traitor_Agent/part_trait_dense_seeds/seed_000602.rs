trait ConstMir { fn const_mir(&self) -> f32; }

impl ConstMir for () {
    fn const_mir(&self) -> f32 { 9007199791611905.0 }
}

fn main() {
    let original = "9007199791611905.0";
    let expected = "9007200000000000";

    let boxed_unit: Box<dyn ConstMir> = Box::new(());
    assert_eq!(boxed_unit.const_mir().to_string(), expected);
    assert_eq!(original.parse::<f32>().unwrap().to_string(), expected);
}