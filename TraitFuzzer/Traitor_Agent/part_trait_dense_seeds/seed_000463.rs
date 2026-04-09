trait AnyExt: std::any::Any {
    fn type_id(&self) -> std::any::TypeId;
}

impl<T> AnyExt for T where T: std::any::Any {
    fn type_id(&self) -> std::any::TypeId {
        std::any::Any::type_id(self)
    }
}

fn main() {
    let x: &dyn AnyExt = &1i32;
    assert_eq!(std::any::Any::type_id(x), std::any::TypeId::of::<i32>());
}