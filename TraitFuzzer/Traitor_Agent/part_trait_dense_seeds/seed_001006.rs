#![feature(impl_trait_in_assoc_type)]

struct SelfReference {
    self_reference: *mut dyn SelfRefSetter,
}

trait SelfRefSetter {
    fn set_self_ref(&mut self);
}

trait SelfRefTrait {
    fn init_self_ref(&mut self) where Self: SelfRefSetter;
}

impl<T> SelfRefTrait for T where T: SelfRefSetter {
    fn init_self_ref(&mut self) {
        self.set_self_ref();
    }
}

impl SelfRefSetter for SelfReference {
    fn set_self_ref(&mut self) {
        self.self_reference = self as *mut dyn SelfRefSetter;
    }
}

fn main() {}